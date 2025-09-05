use crate::types::{AddItemRequest, Cart, Empty, EmptyCartRequest, GetCartRequest};

#[tahini_tarpc::tahini_service(domain=internal)]
pub trait CartService {
    async fn add_item(add_item_req: AddItemRequest);
    async fn get_cart(get_cart_req: GetCartRequest) -> Cart;
    async fn empty_cart(empty_cart_req: EmptyCartRequest);
}
