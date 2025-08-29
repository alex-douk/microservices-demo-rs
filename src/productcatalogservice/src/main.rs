use productcatalog_service::service::ProductCatalogService;

use futures::StreamExt;
use productcatalog_service::types::{ListProductResponse, Product, ProductOut, SearchProductResponse};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use alohomora::pure::PrivacyPureRegion;

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50053);

#[derive(Clone)]
struct ProductCatalogServer {
    catalog: Arc<Vec<ProductOut>>,
}

impl ProductCatalogServer {
    ///We don't do hot reloading of catalog.
    ///Considering Google's original code is reloading the catalog on every request,
    ///and we have no reason to reload the catalog, we simply omit it.
    ///We also do not account for potential AlloyDB integration. We only load locally from file.
    pub fn new() -> Self {
        let catalog_file = File::open("products.json").expect("Couldn't find catalog file");
        let reader = BufReader::new(catalog_file);
        let mut catalog: HashMap<String, Vec<ProductOut>> =
            serde_json::from_reader(reader).expect("Couldn't parse the catalog");
        let products = catalog.remove(&"products".to_string()).expect("Couldn't find products in the catalog");

        Self {
            catalog: Arc::new(products),
        }
    }
}

impl ProductCatalogService for ProductCatalogServer {
    async fn list_products(
        self,
        _context: tarpc::context::Context,
    ) -> productcatalog_service::types::ListProductResponse {
        ListProductResponse {
            products: (*self.catalog).clone(),
        }
    }

    async fn get_product(
        self,
        _context: tarpc::context::Context,
        request: productcatalog_service::types::GetProductRequest,
    ) -> Product {
        let result = request.id.into_ppr(PrivacyPureRegion::new(|id: String| {
            if let Some(prod) = self.catalog.iter().find(|prod| prod.id == id) {
                Some(prod.clone())
            } else {
                None
            }
        }));
        Product::from(result.fold_in().unwrap())
    }

    async fn search_products(
        self,
        _context: tarpc::context::Context,
        request: productcatalog_service::types::SearchProductRequest,
    ) -> productcatalog_service::types::SearchProductResponse {
        let products = request.query.into_ppr(PrivacyPureRegion::new(|query: String| {
            self.catalog
                .iter()
                .filter(|prod| {
                    prod
                        .name
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                        || prod
                            .description
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                })
                .cloned()
                .collect::<Vec<_>>()
        }));

        SearchProductResponse { results: products }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let server = ProductCatalogServer::new();

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
