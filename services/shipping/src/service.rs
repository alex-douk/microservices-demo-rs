use crate::types::{GetQuoteRequest, GetQuoteResponse, ShipOrderRequest, ShipOrderResponse};

#[tarpc::service]
pub trait ShippingService {
    async fn get_quote(quote_req: GetQuoteRequest) -> GetQuoteResponse;
    async fn ship_order(order: ShipOrderRequest) -> ShipOrderResponse;
}
