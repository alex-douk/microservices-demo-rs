use alohomora::bbox::{BBox, BBoxRender};
use alohomora::policy::{AnyPolicyDyn, NoPolicy};
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use alohomora::rocket::BBoxCookieJar;

use crate::middleware::{COOKIE_CURRENCY, COOKIE_SESSION_ID};
use cart_service::types::CartItem;
use currency_service::types::Money;
use productcatalog_service::types::Product;


pub fn current_user_currency(cookie_jar: &BBoxCookieJar<'_, '_>) -> BBox<String, NoPolicy> {
    match cookie_jar.get(COOKIE_CURRENCY) {
        Some(cookie) => cookie.value().to_owned(),
        None => BBox::new("USD".to_string(), NoPolicy {}),
    }
}

#[derive(BBoxRender)]
pub struct ProductView {
    pub item: Product,
    pub price: Money,
}

pub fn user_session_id(cookie_jar: &BBoxCookieJar<'_, '_>) -> BBox<String, NoPolicy> {
    match cookie_jar.get(COOKIE_SESSION_ID) {
        Some(cookie) => cookie.value().to_owned(),
        None => BBox::new("".to_string(), NoPolicy {}),
    }
}

pub fn cart_size(cart: &Vec<CartItem>) -> BBox<i32, NoPolicy> {
    let default = BBox::new(0, NoPolicy {});
    cart.iter().map(|e| e.quantity.clone()).fold(
        default,
        |acc, e| {
            let sum = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
                (acc, e.clone()),
                PrivacyPureRegion::new(|(acc, e): (i32, i64)| {
                    acc + e as i32
                })
            ).unwrap();
            sum.specialize_policy().unwrap()
        }
    )
}
