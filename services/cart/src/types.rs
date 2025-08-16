use tarpc::serde::{Deserialize, Serialize};


pub use microservices_core_types::CartItem;
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CartItem {
//     pub product_id: String,
//     pub quantity: i32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct AddItemRequest {
    pub user_id: String,
    pub item: CartItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyCartRequest {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCartRequest {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn new(user_id: String) -> Self {
        Cart {
            user_id,
            items: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;
