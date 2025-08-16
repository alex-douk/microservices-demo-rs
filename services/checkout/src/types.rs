use tarpc::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderRequest {
    pub user_id: String,
    pub user_currency: String,
    pub address: Address,
    pub email: String,
    pub credit_card: CreditCardInfo

}


#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceOrderResponse {
    pub result: OrderResult
}

//==================REIMPLEMENTING FOREIGN TYPES=================


pub use microservices_core_types::CreditCardInfo;

// //FROM PAYMENT
// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreditCardInfo {
//     pub credit_card_number: String,
//     pub credit_card_cvv: i32,
//     pub credit_card_expiration_year: i32,
//     pub credit_card_expiration_month: i32,
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
//
pub use microservices_core_types::OrderResult;
