use tarpc::serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecommendationsRequest {
    pub user_id: String,
    //Product ids of the items currently in the cart
    pub product_ids: Vec<String>
}




#[derive(Serialize, Deserialize, Debug)]
pub struct ListRecommendationsResponse {
    pub product_ids: Vec<String>
}
