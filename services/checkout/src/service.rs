use crate::types::{PlaceOrderResponse, PlaceOrderRequest};

#[tarpc::service]
pub trait CheckoutService {
    async fn place_order(order_req: PlaceOrderRequest) -> PlaceOrderResponse;

}
