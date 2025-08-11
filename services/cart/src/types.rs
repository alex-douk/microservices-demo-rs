use tarpc::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CartItem {
    pub product_id: String,
    pub quantity: i32,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Cart {
    pub user_id: String,
    pub items: Vec<CartItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty;
