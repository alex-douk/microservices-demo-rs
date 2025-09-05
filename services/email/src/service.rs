use crate::types::{SendOrderConfirmationRequest, UsableSendOrderConfirmationRequest};
use tahini_tarpc::tahini_service;

#[tahini_service(domain=company)]
pub trait EmailService {
    async fn send_order_confirmation(confirmation_request: UsableSendOrderConfirmationRequest);
}
