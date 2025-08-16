use crate::types::{
    GetProductRequest, ListProductResponse, Product, SearchProductRequest, SearchProductResponse,
};

#[tarpc::service]
pub trait ProductCatalogService {
    async fn list_products() -> ListProductResponse;
    async fn get_product(request: GetProductRequest) -> Product;
    async fn search_products(request: SearchProductRequest) -> SearchProductResponse;
}
