use tarpc::serde::{Deserialize, Serialize};

pub use microservices_core_types::OrderResult;
// #[derive(Serialize, Deserialize, Debug)]
// pub struct OrderResult {
//     pub order_id: String,
//     pub shipping_tracking_id: String,
//     pub shipping_cost: Money,
//     pub shipping_address: Address,
//     pub items: Vec<OrderItem>
//
// }

pub use microservices_core_types::OrderItem;
// #[derive(Serialize, Deserialize, Debug)]
// pub struct OrderItem {
//     pub item: CartItem,
//     pub cost: Money,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct SendOrderConfirmationRequest {
    pub email: String,
    pub order: OrderResult,
}

//==================REIMPLEMENTING FOREIGN TYPES=================

pub use microservices_core_types::Money;

// //FROM CURRENCY:
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

pub use microservices_core_types::Address;

// //FROM SHIPPING
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Address {
//     pub street_address: String,
//     pub city: String,
//     pub state: String,
//     pub country: String,
//     pub zip_code: i32
// }

pub use microservices_core_types::CartItem;

// //FROM CART
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CartItem {
//     pub product_id: String,
//     pub quantity: i32,
// }
