use currency_service::service::CurrencyServiceClient;
use currency_service::types::Money;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

use tarpc::serde_transport::new as new_transport;

static CURRENCY_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50056);
static CURRENCY_CLIENT: OnceLock<CurrencyServiceClient> = OnceLock::new();

pub(super) async fn initialize_currency_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&CURRENCY_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = CurrencyServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = CURRENCY_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn convert_currency(
    ctx: tarpc::context::Context,
    from: Money,
    user_currency: String,
) -> Money {
    match CURRENCY_CLIENT.get() {
        None => unreachable!("Currency Client should have been initialized before calling its API"),
        Some(currency_client) => currency_client
            .convert(
                ctx,
                currency_service::types::CurrencyConversionRequest {
                    from,
                    to_code: user_currency,
                },
            )
            .await
            .expect("Couldn't connect to currency client"),
    }
}

pub async fn get_currencies(ctx: tarpc::context::Context) -> Vec<String>{
    match CURRENCY_CLIENT.get() {
        None => unreachable!("Currency Client should have been initialized before calling its API"),
        Some(currency_client) => currency_client
            .get_supported_currencies(ctx).await
            .expect("Couldn't connect to currency client").currency_codes,
    }
}
