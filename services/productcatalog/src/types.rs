use tarpc::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub picture: String,
    pub price_usd: Money,
    pub categories : Vec<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListProductResponse {
    pub products: Vec<Product>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductRequest {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchProductRequest {
    pub query: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SearchProductResponse {
    pub results: Vec<Product>
}
//==================REIMPLEMENTING FOREIGN TYPES=================


pub use microservices_core_types::Money;
// //FROM CURRENCY
// // Represents an amount of money with its currency type.
// #[derive(Serialize, Deserialize, Debug, Clone)]
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
