use crate::middleware::{COOKIE_CURRENCY, COOKIE_SESSION_ID};
use cart_service::types::CartItem;
use currency_service::types::Money;
use productcatalog_service::types::Product;
use rocket::http::CookieJar;

pub fn current_user_currency(cookie_jar: &CookieJar<'_>) -> String {
    let currency_currency = match cookie_jar.get(COOKIE_CURRENCY) {
        Some(cookie) => cookie.value().to_string(),
        None => "USD".to_string(),
    };
    currency_currency
}

#[derive(serde::Serialize)]
pub struct ProductView {
    pub item: Product,
    pub price: Money,
}

pub fn user_session_id(cookie_jar: &CookieJar<'_>) -> String {
    let session_id = match cookie_jar.get(COOKIE_SESSION_ID) {
        Some(cookie) => cookie.value().to_string(),
        None => "".to_string(),
    };
    session_id
}

pub fn cart_size(cart: &Vec<CartItem>) -> i32 {
    cart.iter().map(|e| e.quantity).fold(0, |acc, e| acc + e) 
}
