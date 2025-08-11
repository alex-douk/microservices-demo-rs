use crate::types::{AdRequest, AdResponse};

#[tarpc::service]
pub trait AdService{
    async fn get_ads(request: AdRequest) -> AdResponse;
}
