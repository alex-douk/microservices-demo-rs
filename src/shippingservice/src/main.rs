use rand::{distr::Uniform, Rng};
use shipping_service::{
    service::ShippingService,
    types::{GetQuoteResponse, Money, ShipOrderResponse},
};

use futures::{lock::Mutex, StreamExt};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use futures::Future;
use std::{
    char,
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};
use alohomora::bbox::BBox;
use alohomora::policy::{AnyPolicyDyn, NoPolicy};
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use tarpc::serde_transport::new as new_transport;
use shipping_service::types::{AddressOut, ShipOrderRequestOut};
use crate::db::{backend::MySqlBackend, config::Config};

mod db;
mod quote;

const SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50058);

#[derive(Clone)]
struct ShippingServer(Arc<Mutex<MySqlBackend>>);
impl ShippingServer {
    fn new(config: Config) -> Self {
        ShippingServer(Arc::new(Mutex::new(
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

fn generate_fixed_length_number(nb_digits: u32) -> String {
    let rng = rand::rng();
    rng.sample_iter(Uniform::new_inclusive(48, 57).unwrap())
        .take(nb_digits as usize)
        .map(|e| e as u8 as char)
        .collect()
}
fn generate_tracking(salt: &String) -> String {
    let mut rng = rand::rng();
    let first_letter = rng.random_range(65..=90) as u8 as char;
    let second_letter = rng.random_range(65..=90) as u8 as char;

    let first_len = salt.len();
    let first_number = generate_fixed_length_number(3);

    let second_len = salt.len() / 2;
    let second_number = generate_fixed_length_number(7);
    format!("{first_letter}{second_letter}-{first_len}{first_number}-{second_len}{second_number}")
}

impl ShippingService for ShippingServer {
    async fn get_quote(
        self,
        _context: tarpc::context::Context,
        _quote_req: shipping_service::types::GetQuoteRequest,
    ) -> shipping_service::types::GetQuoteResponse {
        let quote: quote::Quote = 0i32.into();
        let money = Money {
            currency_code: BBox::new("USD".to_string(), NoPolicy {}),
            units: BBox::new(quote.dollars as i64, NoPolicy {}),
            nanos: BBox::new((quote.cents * 10_000_000) as i32, NoPolicy {}),
        };
        GetQuoteResponse { cost_usd: money }
    }
    async fn ship_order(
        self,
        _context: tarpc::context::Context,
        order: shipping_service::types::ShipOrderRequest,
    ) -> shipping_service::types::ShipOrderResponse {
        let tracking_id = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
            order.address.clone(),
            PrivacyPureRegion::new(|address: AddressOut| {
                let salt = format!(
                    "{}, {}, {}",
                    address.street_address, address.city, address.state
                );
                generate_tracking(&salt)
            })
        ).unwrap().specialize_policy().unwrap();

        let mut db_conn = self.0.lock().await;
        let addr = order.address;
        db_conn.insert(
            "orders",
            (
                tracking_id.clone(),
                addr.street_address,
                addr.city,
                addr.state,
                addr.country,
                addr.zip_code,
            ),
            // Need to receive context from tarpc.
            // Do some kind of From tarpc::context::Context -> Context<ContextData>.
            todo!(),
        );

        ShipOrderResponse { tracking_id }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let config = Config::new();
    let server = ShippingServer::new(config);

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
