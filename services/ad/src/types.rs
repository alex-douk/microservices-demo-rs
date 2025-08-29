use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AdRequest {
    // List of important key words from the current page describing the context.
    pub context_keys: BBox<Vec<String>, NoPolicy>,
    pub zip_code: BBox<i32, NoPolicy>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdResponse {
    pub ads: BBox<Vec<Ad>, NoPolicy>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ad {
    // url to redirect to when an ad is clicked.
    pub redirect_url: String,
    // short advertisement text to display.
    pub text: String,
}
