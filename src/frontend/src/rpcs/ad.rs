use ad_service::service::AdServiceClient;
use ad_service::types::Ad;
use cart_service::types::Cart;
use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::serde_transport::new as new_transport;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

static AD_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);
static AD_CLIENT: OnceLock<AdServiceClient> = OnceLock::new();

pub(super) async fn initialize_ad_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&AD_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = AdServiceClient::new(Default::default(), transport).spawn();
    // let a = client.get_ads(ctx, ad_service::types::AdRequest { context_keys: () })
    if let Err(_) = AD_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn get_ad(
    ctxt: tarpc::context::Context,
    context_words: Vec<String>,
    zip_code: i32,
) -> Option<Ad> {
    match AD_CLIENT.get() {
        Some(ad_client) => {
            let ads = ad_client
                .get_ads(
                    ctxt,
                    ad_service::types::AdRequest {
                        context_keys: context_words,
                        zip_code,
                    },
                )
                .await
                .unwrap()
                .ads;
            let mut random = rng();
            ads.choose(&mut random).cloned()
        }
        None => unreachable!("Ad Client should have been initialized before calling its API"),
    }
}
