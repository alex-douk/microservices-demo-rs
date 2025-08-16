use checkout_service::types::CreditCardInfo;
use payment_service::service::PaymentServiceClient;
use payment_service::types::{CreditCardError, Money};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

use tarpc::serde_transport::new as new_transport;

static PAYMENT_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50060);
static PAYMENT_CLIENT: OnceLock<PaymentServiceClient> = OnceLock::new();

pub(super) async fn initialize_payment_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&PAYMENT_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = PaymentServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = PAYMENT_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn charge_card(
    ctx: tarpc::context::Context,
    amount: Money,
    card: CreditCardInfo,
) -> Result<String, CreditCardError> {
    match PAYMENT_CLIENT.get() {
        Some(payment_client) => {
            // let credit_card: PaymentCreditCardInfo = PaymentCreditCardInfo {
            //     credit_card_number: card.credit_card_number,
            //     credit_card_expiration_month: card.credit_card_expiration_month,
            //     credit_card_expiration_year: card.credit_card_expiration_year,
            //     credit_card_cvv: card.credit_card_cvv,
            // };
            payment_client
                .charge(
                    ctx,
                    payment_service::types::ChargeRequest {
                        amount,
                        credit_card: card,
                    },
                )
                .await
                .expect("Couldn't connect to payment client")
                .map(|rsp| rsp.transaction_id)
        }
        // (ctx, cart_service::types::EmptyCartRequest { user_id }).await.expect("Couldn't connect to cart client"),
        None => unreachable!("Cart Client should have been initialized before calling its API"),
    }
}
