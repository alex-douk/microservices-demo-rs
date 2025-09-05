use tahini_tarpc::tahini_service;

use crate::types::{GetQuoteRequest, GetQuoteResponse, ShipOrderRequest, ShipOrderResponse};

#[tahini_service(domain=internal)]
pub trait ShippingService {
    async fn get_quote(quote_req: GetQuoteRequest) -> GetQuoteResponse;
    async fn ship_order(order: ShipOrderRequest) -> ShipOrderResponse;
}
