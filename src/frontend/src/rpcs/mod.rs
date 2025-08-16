pub mod ad;
pub mod cart;
pub mod catalog;
pub mod checkout;
pub mod currency;
pub mod recommendation;
pub mod shipping;


pub async fn init_services() {
    // ad::initialize_email_client().await;
    cart::initialize_cart_client().await;
    catalog::initialize_catalog_client().await;
    checkout::initialize_checkout_client().await;
    currency::initialize_currency_client().await;
    recommendation::initialize_recommendation_client().await;
    shipping::initialize_shipping_client().await;
}
