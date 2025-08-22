use checkout_service::service::CheckoutService;
use checkout_service::types::PlaceOrderResponse;
use email_service::types::{Money, OrderItem, OrderResult};
use futures::StreamExt;
use mysql::serde_json;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use uuid::Uuid;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

use crate::db::backend::MySqlBackend;
use crate::db::config::Config;
use crate::dependent_services::cart::{delete_cart, get_cart};
use crate::dependent_services::catalog::prepare_order;
use crate::dependent_services::currency::convert_currency;
use crate::dependent_services::email::send_order_confirmation;
use crate::dependent_services::payment::charge_card;
use crate::dependent_services::shipping::{get_quote, ship_order};
use crate::money::sum;

mod db;
mod dependent_services;
mod money;

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50059);

#[derive(Clone)]
struct CheckoutServer(Arc<Mutex<MySqlBackend>>);

fn store_order_to_db(
    db_conn: &mut MySqlBackend,
    order_id: &str,
    items: Vec<OrderItem>,
    tx_id: &str,
    tracking_id: &str,
) {
    let formated_items = items
        .iter()
        .map(|item: &OrderItem| {
            let str_price = serde_json::to_string(&item.cost).expect("Couldn't serialize price");
            (
                None::<u8>,
                order_id,
                item.item.product_id.to_string(),
                item.item.quantity,
                str_price,
            )
        })
        .collect::<Vec<_>>();
    db_conn.insert("checkout_orders", (order_id, tx_id, tracking_id));
    db_conn.multiple_insert("ordered_items", formated_items);
}

impl CheckoutServer {
    fn new(config: Config) -> Self {
        CheckoutServer(Arc::new(Mutex::new(
            MySqlBackend::new(
                config.username.as_str(),
                config.password.as_str(),
                config.database.as_str(),
                config.prime,
            )
            .expect("Couldn't connect to DB"),
        )))
    }
}

impl CheckoutService for CheckoutServer {
    async fn place_order(
        self,
        context: tarpc::context::Context,
        order_req: checkout_service::types::PlaceOrderRequest,
    ) -> checkout_service::types::PlaceOrderResponse {
        let uuid = Uuid::new_v4().to_string();
        let cart = get_cart(context, order_req.user_id.clone()).await;
        let order =
            prepare_order(context, cart.items.clone(), order_req.user_currency.clone()).await;
        let shipping_cost_usd =
            get_quote(context, order_req.address.clone(), cart.items.clone()).await;
        let shipping_cost_localized =
            convert_currency(context, shipping_cost_usd, order_req.user_currency.clone()).await;

        let mut total = Money {
            units: 0,
            nanos: 0,
            currency_code: order_req.user_currency,
        };

        total = money::sum(&total, &shipping_cost_localized).expect("Shipping costs are malformed");
        let total_price = order
            .iter()
            // .iter_mut()
            .map(|order_item| money::slow_multiply(&order_item.cost, order_item.item.quantity))
            .try_fold(total, |acc, cost| sum(&acc, &cost))
            // .try_reduce(|acc, cost| money::sum(&acc, &cost))
            .expect("Item price is malformed");
        let tx_id = charge_card(
            context,
            total_price,
            order_req.credit_card,
            order_req.save_payment_info,
        )
        .await
        .expect("Credit card error");

        let tracking_id = ship_order(context, order_req.address.clone(), cart.items).await;

        let mut db_conn = self.0.lock().await;
        store_order_to_db(
            &mut *db_conn,
            uuid.as_str(),
            order.clone(),
            tx_id.as_str(),
            tracking_id.as_str(),
        );

        delete_cart(context, order_req.user_id).await;

        let order_result = OrderResult {
            order_id: uuid,
            shipping_tracking_id: tracking_id,
            shipping_cost: shipping_cost_localized,
            shipping_address: order_req.address,
            items: order,
        };

        send_order_confirmation(context, order_req.email, order_result.clone()).await;

        PlaceOrderResponse {
            result: order_result,
        }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    //Initialize connections to all services (although the Google code creates a new connection per
    //request!!)
    dependent_services::init_services().await;
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let config = Config::new();
    let server = CheckoutServer::new(config);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let framed = codec_builder.new_framed(stream);
        let transport = new_transport(framed, Json::default());
        let fut = BaseChannel::with_defaults(transport)
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
