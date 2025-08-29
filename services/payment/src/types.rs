use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::serde::{Serialize, Deserialize};



pub use microservices_core_types::{CreditCardInfo, CreditCardInfoOut};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CreditCardInfo {
//     pub credit_card_number: String,
//     pub credit_card_cvv: i32,
//     pub credit_card_expiration_year: i32,
//     pub credit_card_expiration_month: i32,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargeRequest {
    pub amount: Money,
    pub credit_card: CreditCardInfo,
    pub save_credit_info: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargeResponse {
    pub transaction_id: BBox<String, NoPolicy>,
}



//==================REIMPLEMENTING FOREIGN TYPES=================


pub use microservices_core_types::Money;
// //FROM CURRENCY:
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CreditCardError {
    InvalidCreditCard,
    UnnaceptedCreditCard(String),
    ExpiredCreditCard(String, i32, i32),
}

impl std::fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self {
            Self::InvalidCreditCard => "Credit card info is invalid".to_string(),
            Self::UnnaceptedCreditCard(ref card_type) => format!("Sorry, we cannot process {card_type} credit cards. Only VISA or Mastercard is accepted"),
            Self::ExpiredCreditCard(
                end_of_number, month, year) => format!("You credit card (ending {}) expired on {}/{}", end_of_number, month, year)
        };
        write!(f, "{}", data)
    }
}

impl std::error::Error for CreditCardError {}
