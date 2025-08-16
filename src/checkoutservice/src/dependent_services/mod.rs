pub mod cart;
pub mod email;
pub mod catalog;
pub mod currency;
pub mod payment;
pub mod shipping;


pub async fn init_services() {
    cart::initialize_cart_client().await;
    email::initialize_email_client().await;
    catalog::initialize_catalog_client().await;
    currency::initialize_currency_client().await;
    payment::initialize_payment_client().await;
    shipping::initialize_shipping_client().await;
}
