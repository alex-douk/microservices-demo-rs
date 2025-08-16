mod validate;

use futures::StreamExt;
use payment_service::service::PaymentService;
use payment_service::types::ChargeResponse;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use uuid::Uuid;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50060);

#[derive(Clone)]
struct PaymentServer;

impl PaymentService for PaymentServer {
    async fn charge(
        self,
        _context: tarpc::context::Context,
        charge: payment_service::types::ChargeRequest,
    ) -> Result<payment_service::types::ChargeResponse, payment_service::types::CreditCardError>
    {
        let details = validate::validate_card(charge.credit_card)?;
        let amount = charge.amount;
        println!(
            "Transaction processed: {} ending {}\
        Amount: {}{}.{}",
            details.card_type,
            details.end_numbers,
            amount.currency_code,
            amount.units,
            amount.nanos
        );
        Ok(ChargeResponse {
            transaction_id: Uuid::new_v4().to_string(),
        })
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let server = PaymentServer;

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
