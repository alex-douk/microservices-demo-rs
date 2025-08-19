use checkout_service::service::CheckoutService;
use checkout_service::types::PlaceOrderResponse;
use email_service::types::{Money, OrderResult};
use futures::StreamExt;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use uuid::Uuid;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

use crate::dependent_services::cart::{delete_cart, get_cart};
use crate::dependent_services::catalog::prepare_order;
use crate::dependent_services::currency::convert_currency;
use crate::dependent_services::email::send_order_confirmation;
use crate::dependent_services::payment::charge_card;
use crate::dependent_services::shipping::{get_quote, ship_order};
use crate::money::sum;

mod dependent_services;
mod money;

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50059);

#[derive(Clone)]
struct CheckoutServer;

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
        let _ = charge_card(
            context,
            total_price,
            order_req.credit_card,
            order_req.save_payment_info,
        )
        .await
        .expect("Credit card error");

        let tracking_id = ship_order(context, order_req.address.clone(), cart.items).await;

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
    let server = CheckoutServer;

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
