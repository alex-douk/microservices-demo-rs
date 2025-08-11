use tarpc::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreditCardInfo {
    pub credit_card_number: String,
    pub credit_card_cvv: i32,
    pub credit_card_expiration_year: i32,
    pub credit_card_expiration_month: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargeRequest {
    pub amount: Money,
    pub credit_card: CreditCardInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargeResponse {
    transaction_id: String
}



//==================REIMPLEMENTING FOREIGN TYPES=================

//FROM CURRENCY:
#[derive(Serialize, Deserialize, Debug)]
pub struct Money {
    // The 3-letter currency code defined in ISO 4217.
    pub currency_code: String,
    //The whole units of the amount.
    // For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: i64,
    //Number of nano (10^-9) units of the amount.
    // The value must be between -999,999,999 and +999,999,999 inclusive.
    // If `units` is positive, `nanos` must be positive or zero.
    // If `units` is zero, `nanos` can be positive, zero, or negative.
    // If `units` is negative, `nanos` must be negative or zero.
    // For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: i32,
}
