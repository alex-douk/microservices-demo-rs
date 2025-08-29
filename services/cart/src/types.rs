use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::serde::{Deserialize, Serialize};


pub use microservices_core_types::CartItem;
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CartItem {
//     pub product_id: String,
//     pub quantity: i32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct AddItemRequest {
    pub user_id: BBox<String, NoPolicy>,
    pub item: BBox<CartItem, NoPolicy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyCartRequest {
    pub user_id: BBox<String, NoPolicy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCartRequest {
    pub user_id: BBox<String, NoPolicy>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    pub user_id: BBox<String, NoPolicy>,
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn new(user_id: BBox<String, NoPolicy>) -> Self {
        Cart {
            user_id,
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;
