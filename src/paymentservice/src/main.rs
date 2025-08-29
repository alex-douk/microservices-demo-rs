mod db;
mod validate;

use futures::lock::Mutex;
use futures::StreamExt;
use payment_service::service::PaymentService;
use payment_service::types::{ChargeResponse, CreditCardInfoOut};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use alohomora::bbox::BBox;
use alohomora::pcr::{execute_pcr, PrivacyCriticalRegion, Signature};
use alohomora::policy::{AnyPolicyCloneDyn, AnyPolicyDyn, NoPolicy};
use alohomora::pure::PrivacyPureRegion;
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use uuid::Uuid;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

use crate::db::backend::MySqlBackend;
use crate::db::config::Config;
use crate::validate::CreditCardDetails;

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
        // This is a critical region because in reality, it speaks to a remote payment processor.
        let details = execute_pcr::<dyn AnyPolicyCloneDyn, _, _, _, _>(
            charge.credit_card.clone(),
            PrivacyCriticalRegion::new(
                |credit_card: CreditCardInfoOut, p, _: ()| {
                    BBox::new(validate::validate_card(credit_card), p)
                },
                Signature {
                    username: "",
                    signature: "",
                }
            ),
            ()
        ).unwrap().fold_in()?;

        // TODO(babman): probably a good idea to write the amount to the DB.
        let amount = charge.amount;
        /*
        println!(
            "Transaction processed: {} ending {}\
        Amount: {}{}.{}",
            details.card_type,
            details.end_numbers,
            amount.currency_code,
            amount.units,
            amount.nanos
        );
         */

        let tx_id = BBox::new(Uuid::new_v4().to_string(), NoPolicy {});
        if charge.save_credit_info {
            let mut db_conn = self.0.lock().await;
            db_conn.insert(
                "payments",
                (
                    tx_id.clone(),
                    details.into_ppr(PrivacyPureRegion::new(|d: CreditCardDetails| d.card_type)),
                    charge.credit_card.credit_card_number,
                    charge.credit_card.credit_card_expiration_month,
                    charge.credit_card.credit_card_expiration_year,
                    charge.credit_card.credit_card_cvv.into_ppr(PrivacyPureRegion::new(|x: i32| x.to_string())),
                ),
                todo!()
            );
        }

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
