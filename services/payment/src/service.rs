use crate::types::{ChargeRequest, ChargeResponse, PaymentChargeRequest};
use tahini_tarpc::tahini_service;

#[tahini_service(domain=company)]
pub trait PaymentService {
    async fn charge(charge: PaymentChargeRequest) -> Result<ChargeResponse, String>;
}
