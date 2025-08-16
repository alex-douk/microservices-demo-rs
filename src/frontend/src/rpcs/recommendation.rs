// use productcatalog_service::service::ProductCatalogServiceClient;
use productcatalog_service::types::Product;
use recommendation_service::service::RecommendationServiceClient;
use std::cmp::min;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

use tarpc::serde_transport::new as new_transport;

use crate::rpcs::catalog::get_product;

static RECOMMENDATION_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
static RECOMMENDATION_CLIENT: OnceLock<RecommendationServiceClient> = OnceLock::new();

pub(super) async fn initialize_recommendation_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&RECOMMENDATION_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = RecommendationServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = RECOMMENDATION_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn list_recommendations(
    ctx: tarpc::context::Context,
    user_id: String,
    product_ids: Vec<String>,
) -> Vec<Product> {
    match RECOMMENDATION_CLIENT.get() {
        None => unreachable!("Catalog Client should have been initialized before calling its API"),
        Some(recommendation_client) => {
            let product_ids = recommendation_client
                .list_recommendations(
                    ctx,
                    recommendation_service::types::ListRecommendationsRequest {
                        user_id,
                        product_ids,
                    },
                )
                .await
                .expect("Couldn't connect to catalog client")
                .product_ids
                .into_iter()
                .take(4);

            let mut recommendations: Vec<Product> = Vec::with_capacity(min(product_ids.len(), 4));
            for id in product_ids {
                recommendations.push(get_product(ctx, id).await);
            }
            recommendations
        }
    }
}
