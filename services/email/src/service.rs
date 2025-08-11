#[tarpc::service]
pub trait EmailService {
    async fn send_order_confirmation();
}
