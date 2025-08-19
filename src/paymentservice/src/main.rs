mod db;
mod validate;

use futures::lock::Mutex;
use futures::StreamExt;
use payment_service::service::PaymentService;
use payment_service::types::ChargeResponse;
use std::char;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use uuid::Uuid;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

use crate::db::backend::MySqlBackend;
use crate::db::config::Config;

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50060);

#[derive(Clone)]
struct PaymentServer(Arc<Mutex<MySqlBackend>>);

impl PaymentServer {
    fn new(config: Config) -> Self {
        PaymentServer(Arc::new(Mutex::new(
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

impl PaymentService for PaymentServer {
    async fn charge(
        self,
        _context: tarpc::context::Context,
        charge: payment_service::types::ChargeRequest,
    ) -> Result<payment_service::types::ChargeResponse, payment_service::types::CreditCardError>
    {
        let details = validate::validate_card(charge.credit_card.clone())?;
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

        let mut db_conn = self.0.lock().await;
        let tx_id = Uuid::new_v4().to_string();
        db_conn.insert(
            "payments",
            (
                tx_id.clone(),
                details.card_type,
                charge.credit_card.credit_card_number,
                charge.credit_card.credit_card_expiration_month,
                charge.credit_card.credit_card_expiration_year,
                charge.credit_card.credit_card_cvv.to_string(),
            ),
        );

        Ok(ChargeResponse {
            transaction_id: tx_id,
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
    let config = Config::new();
    let server = PaymentServer::new(config);

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
