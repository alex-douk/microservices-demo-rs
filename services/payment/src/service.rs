use crate::types::{ChargeRequest, ChargeResponse, CreditCardError, };

#[tarpc::service]
pub trait PaymentService {
    async fn charge(charge: ChargeRequest) -> Result<ChargeResponse, CreditCardError>;
}
