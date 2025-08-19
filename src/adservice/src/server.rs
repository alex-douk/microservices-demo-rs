use multimap::MultiMap;
use std::sync::Arc;

use ad_service::{
    service::AdService,
    types::{Ad, AdResponse},
};

use futures::StreamExt;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

#[derive(Clone)]
pub struct AdServer {
    ads_map: Arc<MultiMap<String, Ad>>,
}

impl AdServer {
    pub fn new() -> Self {
        Self {
            ads_map: Arc::new(AdServer::create_ads_map()),
        }
    }

    fn create_ads_map() -> MultiMap<String, Ad> {
        // let mut ad_map = HashMap::new();
        let mut ad_map = MultiMap::new();
        let hairdyer = Ad {
            redirect_url: "/product/2ZYFJ3GM2N".to_string(),
            text: "Hairdryer for sale. 50% off.".to_string(),
        };

        let tank_top = Ad {
            redirect_url: "/product/66VCHSJNUP".to_string(),
            text: "Tank top for sale. 20% off.".to_string(),
        };

        let candle_holder = Ad {
            redirect_url: "/product/0PUK6V6EV0".to_string(),
            text: "Candle holder for sale. 30% off.".to_string(),
        };

        let bamboo_glass_jar = Ad {
            redirect_url: "/product/9SIQT8TOJO".to_string(),
            text: "Bamboo glass jar for sale. 10% off.".to_string(),
        };

        let watch = Ad {
            redirect_url: "/product/1YMWWN1N4O".to_string(),
            text: "Watch for sale. Buy one, get second kit for free.".to_string(),
        };

        let mug = Ad {
            redirect_url: "/product/6E92ZMYYFZ".to_string(),
            text: "Mug for sale. Buy two, get third one for free.".to_string(),
        };

        let loafers = Ad {
            redirect_url: "/product/L9ECAV7KIM".to_string(),
            text: "Loafers for sale. Buy one, get second one for free".to_string(),
        };

        ad_map.insert("clothing".to_string(), tank_top);
        ad_map.insert("accessories".to_string(), watch);
        ad_map.insert("footwear".to_string(), loafers);
        ad_map.insert("hair".to_string(), hairdyer);
        ad_map.insert("decor".to_string(), candle_holder);
        ad_map.insert("kitchen".to_string(), bamboo_glass_jar);
        ad_map.insert("kitchen".to_string(), mug);
        ad_map
    }
}

impl AdService for AdServer {
    async fn get_ads(
        self,
        _context: ::tarpc::context::Context,
        request: ad_service::types::AdRequest,
    ) -> ad_service::types::AdResponse {
        println!("Getting ads based on zipcode {:06}", request.zip_code);
        let ads = request
            .context_keys
            .iter()
            .map(|context_key| {
                self.ads_map
                    .get_vec(context_key)
                    .cloned()
                    .unwrap_or_else(|| Vec::new())
            })
            .flatten()
            .collect::<Vec<_>>();

        AdResponse { ads }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let server = AdServer::new();
    let addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);
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
