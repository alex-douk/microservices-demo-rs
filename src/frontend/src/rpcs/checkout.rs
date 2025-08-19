use checkout_service::service::CheckoutServiceClient;
use checkout_service::types::{Address, CreditCardInfo, PlaceOrderRequest};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

static CHECKOUT_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50059);
static CHECKOUT_CLIENT: OnceLock<CheckoutServiceClient> = OnceLock::new();

pub(super) async fn initialize_checkout_client() {
    println!("INTIIALIZNG CHECKOUT SERVICE");
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&CHECKOUT_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = CheckoutServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = CHECKOUT_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn checkout(
    ctx: tarpc::context::Context,
    address: Address,
    email: String,
    cc: CreditCardInfo,
    session_id: String,
    currency: String,
    save_payment_info: bool
) -> checkout_service::types::OrderResult {
    match CHECKOUT_CLIENT.get() {
        Some(checkout_client) => {
            let order = PlaceOrderRequest {
                user_id: session_id,
                user_currency: currency,
                address,
                email,
                credit_card: cc,
                save_payment_info

            };
            checkout_client
                .place_order(ctx, order)
                .await
                .expect("Couldn't connect to checkout client")
                .result
        }
        None => unreachable!("Cart Client should have been initialized before calling its API"),
    }
}
