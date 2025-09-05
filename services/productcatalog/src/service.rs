use crate::types::{
    GetProductRequest, ListProductResponse, Product, SearchProductRequest, SearchProductResponse,
};
use tahini_tarpc::tahini_service;


#[tahini_service(domain=company)]
pub trait ProductCatalogService {
    async fn list_products() -> ListProductResponse;
    async fn get_product(request: GetProductRequest) -> Product;
    async fn search_products(request: SearchProductRequest) -> SearchProductResponse;
}
