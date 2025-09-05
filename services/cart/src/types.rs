use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use alohomora::SesameType;
use tahini_tarpc::TahiniType;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};


pub use microservices_core_types::CartItem;
// #[derive(Serialize, TahiniDeserialize, Debug, Clone)]
// pub struct CartItem {
//     pub product_id: String,
//     pub quantity: i32,
// }

#[derive(TahiniType, TahiniDeserialize, Debug, Clone)]
pub struct AddItemRequest {
    pub user_id: BBox<String, NoPolicy>,
    pub item: CartItem,
}

#[derive(TahiniType, TahiniDeserialize, Debug, Clone)]
pub struct EmptyCartRequest {
    pub user_id: BBox<String, NoPolicy>,
}

#[derive(TahiniType, TahiniDeserialize, Debug, Clone)]
pub struct GetCartRequest {
    pub user_id: BBox<String, NoPolicy>,
}

#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType)]
#[alohomora_out_type(to_derive = [Clone])]
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

#[derive(TahiniSerialize, TahiniDeserialize, Debug)]
pub struct Empty;
