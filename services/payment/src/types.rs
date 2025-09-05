use tahini_tarpc::{TahiniTransformInto, TahiniType};
use alohomora::{bbox::BBox, SesameType};
use alohomora::policy::NoPolicy;
use tahini_tarpc::{TahiniSerialize, TahiniDeserialize};
pub use microservices_core_types::Money;
use crate::policy::{MoneyProcessingPolicy, PaymentProcessingPolicy};



pub use microservices_core_types::{CreditCardInfo, CreditCardInfoOut};

// #[derive(Serialize, TahiniDeserialize, Debug)]
// pub struct CreditCardInfo {
//     pub credit_card_number: String,
//     pub credit_card_cvv: i32,
//     pub credit_card_expiration_year: i32,
//     pub credit_card_expiration_month: i32,
// }

#[derive(Clone, TahiniDeserialize, Debug, TahiniType)]
pub struct ChargeRequest {
    pub amount: Money,
    pub credit_card: CreditCardInfo
}


#[derive(Clone, TahiniDeserialize, Debug, TahiniType)]
pub struct PaymentChargeRequest {
    pub amount: Money,
    pub credit_card: PaymentCreditCardInfo
}

impl TahiniTransformInto<PaymentChargeRequest> for ChargeRequest {
    fn transform_into(self, context: &tahini_tarpc::context::TahiniContext) -> Result<PaymentChargeRequest, String> {
        Ok(PaymentChargeRequest {
            amount: self.amount,
            credit_card: self.credit_card.transform_into(context)?
        })
    }

}




#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType)]
#[alohomora_out_type(to_derive=[TahiniSerialize, TahiniDeserialize, Debug, Clone])]
pub struct PaymentCreditCardInfo {
    pub credit_card_number: BBox<String, PaymentProcessingPolicy>,
    pub credit_card_cvv: BBox<i32, PaymentProcessingPolicy>,
    pub credit_card_expiration_year: BBox<i32, PaymentProcessingPolicy>,
    pub credit_card_expiration_month: BBox<i32, PaymentProcessingPolicy>,
}


impl TahiniTransformInto<PaymentCreditCardInfo> for CreditCardInfo {
    fn transform_into(self, context: &tahini_tarpc::context::TahiniContext) -> Result<PaymentCreditCardInfo, String> {
        Ok(PaymentCreditCardInfo {
            credit_card_number: self.credit_card_number.transform_into(context)?,
            credit_card_expiration_year: self.credit_card_expiration_year.transform_into(context)?,
            credit_card_expiration_month: self.credit_card_expiration_month.transform_into(context)?,
            credit_card_cvv: self.credit_card_cvv.transform_into(context)?
        })
    }

}


#[derive(TahiniType, Clone, TahiniDeserialize, Debug)]
pub struct ChargeResponse {
    pub transaction_id: BBox<String, NoPolicy>,
}




//==================REIMPLEMENTING FOREIGN TYPES=================


#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType)]
#[alohomora_out_type(to_derive=[TahiniSerialize, TahiniDeserialize, Debug, Clone])]
pub struct PaymentMoney {
    // The 3-letter currency code defined in ISO 4217.
    pub currency_code: BBox<i64, NoPolicy>,
    //The whole units of the amount.
    // For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    pub units: BBox<i64, NoPolicy>,
    //Number of nano (10^-9) units of the amount.
    // The value must be between -999,999,999 and +999,999,999 inclusive.
    // If `units` is positive, `nanos` must be positive or zero.
    // If `units` is zero, `nanos` can be positive, zero, or negative.
    // If `units` is negative, `nanos` must be negative or zero.
    // For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    pub nanos: BBox<i64, NoPolicy>,
}

// #[derive(TahiniType, TahiniDeserialize, Clone, Debug)]
// pub enum CreditCardError {
//     InvalidCreditCard,
//     UnnaceptedCreditCard(String),
//     ExpiredCreditCard(String, i32, i32),
// }

// impl std::fmt::Display for CreditCardError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let data = match self {
//             Self::InvalidCreditCard => "Credit card info is invalid".to_string(),
//             Self::UnnaceptedCreditCard(ref card_type) => format!("Sorry, we cannot process {card_type} credit cards. Only VISA or Mastercard is accepted"),
//             Self::ExpiredCreditCard(
//                 end_of_number, month, year) => format!("You credit card (ending {}) expired on {}/{}", end_of_number, month, year)
//         };
//         write!(f, "{}", data)
//     }
// }

// impl std::error::Error for CreditCardError {}
