use crate::types::{ChargeRequest, ChargeResponse};

#[tarpc::service]
pub trait PaymentService {
    async fn charge(charge: ChargeRequest) -> ChargeResponse;
}
