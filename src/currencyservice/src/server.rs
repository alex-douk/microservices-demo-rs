use currency_service::service::CurrencyService;
use currency_service::{self, types::GetSupportedCurrenciesResponse};
use futures::StreamExt;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use crate::currency_utils::convert::ConversionTable;
use futures::Future;
use tarpc::serde_transport::new as new_transport;

mod currency_utils;

#[derive(Clone)]
struct CurrencyServer {
    currency_converter: Arc<currency_utils::convert::ConversionTable>,
}

impl currency_service::service::CurrencyService for CurrencyServer {
    async fn convert(
        self,
        _context: tarpc::context::Context,
        conversion_request: currency_service::types::CurrencyConversionRequest,
    ) -> currency_service::types::Money {
        self.currency_converter
            .convert(conversion_request.from, conversion_request.to_code)
    }

    async fn get_supported_currencies(
        self,
        _context: tarpc::context::Context,
    ) -> currency_service::types::GetSupportedCurrenciesResponse {
        GetSupportedCurrenciesResponse {
            currency_codes: self
                .currency_converter
                .table
                .keys()
                .map(|str_ref: &String| str_ref.clone())
                .collect(),
        }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let server = CurrencyServer {
        currency_converter: Arc::new(ConversionTable::new()),
    };

    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50056);
    let listener = TcpListener::bind(&addr).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();

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
