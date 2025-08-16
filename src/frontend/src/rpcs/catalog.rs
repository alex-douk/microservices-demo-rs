use productcatalog_service::service::ProductCatalogServiceClient;
use productcatalog_service::types::Product;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

use tarpc::serde_transport::new as new_transport;

static CATALOG_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50053);
static CATALOG_CLIENT: OnceLock<ProductCatalogServiceClient> = OnceLock::new();

pub(super) async fn initialize_catalog_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&CATALOG_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = ProductCatalogServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = CATALOG_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn get_product(ctx: tarpc::context::Context, product_id: String) -> Product {
    match CATALOG_CLIENT.get() {
        None => unreachable!("Catalog Client should have been initialized before calling its API"),
        Some(catalog_client) => catalog_client
            .get_product(
                ctx,
                productcatalog_service::types::GetProductRequest { id: product_id },
            )
            .await
            .expect("Couldn't connect to catalog client"),
    }
}


pub async fn list_products(ctx: tarpc::context::Context) -> Vec<Product> {
    match CATALOG_CLIENT.get() {
        None => unreachable!("Catalog Client should have been initialized before calling its API"),
        Some(catalog_client) => catalog_client
            .list_products(ctx)
            .await
            .expect("Couldn't connect to catalog client").products,
    }
}
