use crate::types::SendOrderConfirmationRequest;

#[tarpc::service]
pub trait EmailService {
    async fn send_order_confirmation(confirmation_request: SendOrderConfirmationRequest);
}
