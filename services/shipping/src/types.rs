use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use alohomora::SesameType;
use tarpc::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetQuoteRequest {
    pub address: Address,
    pub items: Vec<CartItem>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetQuoteResponse {
    pub cost_usd: Money
}


#[derive(Serialize, Deserialize, Debug, Clone, SesameType)]
pub struct ShipOrderRequest {
    pub address: Address,
    pub items: Vec<CartItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipOrderResponse {
    pub tracking_id: BBox<String, NoPolicy>,
}


pub use microservices_core_types::{Address, AddressOut};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Address {
//     pub street_address: String,
//     pub city: String,
//     pub state: String,
//     pub country: String,
//     pub zip_code: i32
// }


//==================REIMPLEMENTING FOREIGN TYPES=================



pub use microservices_core_types::CartItem;
pub use microservices_core_types::Money;

// //FROM CART
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CartItem {
//     pub product_id: String,
//     pub quantity: i32,
// }
//
//
// //FROM CURRENCY
// // Represents an amount of money with its currency type.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Money {
//     // The 3-letter currency code defined in ISO 4217.
//     pub currency_code: String,
//     //The whole units of the amount.
//     // For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
//     pub units: i64,
//     //Number of nano (10^-9) units of the amount.
//     // The value must be between -999,999,999 and +999,999,999 inclusive.
//     // If `units` is positive, `nanos` must be positive or zero.
//     // If `units` is zero, `nanos` can be positive, zero, or negative.
//     // If `units` is negative, `nanos` must be negative or zero.
//     // For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
//     pub nanos: i32,
// }
