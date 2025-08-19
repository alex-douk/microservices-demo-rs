use tarpc::serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct AdRequest {
    // List of important key words from the current page describing the context.
    pub context_keys: Vec<String>,
    pub zip_code: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdResponse {
    pub ads: Vec<Ad>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ad {
    // url to redirect to when an ad is clicked.
    pub redirect_url: String,
    // short advertisement text to display.
    pub text: String,
}
