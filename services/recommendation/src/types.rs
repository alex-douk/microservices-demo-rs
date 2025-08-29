use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecommendationsRequest {
    pub user_id: BBox<String, NoPolicy>,
    //Product ids of the items currently in the cart
    pub product_ids: BBox<Vec<String>, NoPolicy>,
}




#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecommendationsResponse {
    pub product_ids: BBox<Vec<String>, NoPolicy>,
}
