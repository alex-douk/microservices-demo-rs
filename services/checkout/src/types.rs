use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use microservices_core_types::policies::{EmailAddressPolicy, MoneyPolicy};
use tahini_tarpc::TahiniTransformInto;
use tahini_tarpc::{TahiniType, TahiniDeserialize};


#[derive(TahiniType, TahiniDeserialize, Clone, Debug)]
pub struct PlaceOrderRequest {
    pub user_id: BBox<String, NoPolicy>,
    pub user_currency: BBox<String, NoPolicy>,
    pub address: Address,
    pub email: BBox<String, EmailAddressPolicy>,
    pub credit_card: CreditCardInfo,
    pub save_payment_info: bool,
}

impl TahiniTransformInto<PlaceOrderRequest> for PlaceOrderRequest {
    fn transform_into(self, context: &tahini_tarpc::context::TahiniContext) -> Result<PlaceOrderRequest, String> {
        Ok(self)
    }
}


#[derive(TahiniType, TahiniDeserialize, Debug, Clone)]
pub struct PlaceOrderResponse {
    pub result: OrderResult
}

//==================REIMPLEMENTING FOREIGN TYPES=================


pub use microservices_core_types::CreditCardInfo;

// //FROM PAYMENT
// #[derive(Serialize, TahiniDeserialize, Debug)]
// pub struct CreditCardInfo {
//     pub credit_card_number: String,
//     pub credit_card_cvv: i32,
//     pub credit_card_expiration_year: i32,
//     pub credit_card_expiration_month: i32,
// }



pub use microservices_core_types::Address;

// //FROM SHIPPING
// #[derive(Serialize, TahiniDeserialize, Debug)]
// pub struct Address {
//     pub street_address: String,
//     pub city: String,
//     pub state: String,
//     pub country: String,
//     pub zip_code: i32
// }
//
pub use microservices_core_types::OrderResult;
