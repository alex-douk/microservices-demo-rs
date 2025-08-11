#[tarpc::service]
pub trait SERVICE {
    async fn test();
}
