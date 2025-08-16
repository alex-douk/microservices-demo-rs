use productcatalog_service::service::ProductCatalogServiceClient;
use rand::seq::IteratorRandom;
use recommendation_service::service::RecommendationService;

use futures::StreamExt;
use recommendation_service::types::ListRecommendationsResponse;
use std::cmp::min;
use std::collections::HashSet;
use std::hash::RandomState;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, OnceLock};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::{TcpListener, TcpStream};

use futures::Future;
use tarpc::serde_transport::new as new_transport;

static CATALOG_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50053);
static CATALOG_CLIENT: OnceLock<ProductCatalogServiceClient> = OnceLock::new();

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);

async fn initialize_catalog_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&CATALOG_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = ProductCatalogServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = CATALOG_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

#[derive(Clone)]
struct RecommendationServer;

impl RecommendationService for RecommendationServer {
    async fn list_recommendations(
        self,
        context: tarpc::context::Context,
        request: recommendation_service::types::ListRecommendationsRequest,
    ) -> recommendation_service::types::ListRecommendationsResponse {
        let max_responses = 5;
        //fetch list of products from product catalog stub
        let catalog = CATALOG_CLIENT
            .get()
            .unwrap()
            .list_products(context)
            .await
            .expect("Couldn't get the catalog")
            .products;
        let hash_set: HashSet<String, RandomState> = HashSet::from_iter(request.product_ids);
        let sampled_ids = catalog
            .into_iter()
            .map(|prod| prod.id)
            .filter(|prod_id| !hash_set.contains(prod_id))
            .collect::<Vec<_>>();
        let nb_samples = min(max_responses, sampled_ids.len());
        let mut rng = rand::rng();

        ListRecommendationsResponse {
            product_ids: sampled_ids
                .into_iter()
                .choose_multiple(&mut rng, nb_samples),
        }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    initialize_catalog_client().await;
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let server = RecommendationServer;

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
