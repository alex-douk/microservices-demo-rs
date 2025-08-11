use crate::types::{ListRecommendationsRequest, ListRecommendationsResponse};

#[tarpc::service]
pub trait RecommendationService {
    async fn list_recommendations(
        request: ListRecommendationsRequest,
    ) -> ListRecommendationsResponse;
}
