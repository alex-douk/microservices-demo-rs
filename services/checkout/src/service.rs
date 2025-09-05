use tahini_tarpc::tahini_service;

use crate::types::{PlaceOrderResponse, PlaceOrderRequest};

#[tahini_service(domain=company)]
pub trait CheckoutService {
    async fn place_order(order_req: PlaceOrderRequest) -> PlaceOrderResponse;
}
