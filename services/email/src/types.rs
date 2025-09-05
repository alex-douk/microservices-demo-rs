use alohomora::bbox::BBox;
use alohomora::policy::{NoPolicy, SimplePolicy};
use alohomora::SesameType;
use microservices_core_types::policies::EmailAddressPolicy;
use tahini_tarpc::traits::PolicyInto;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use tahini_tarpc::{TahiniTransformInto, TahiniType};
use tarpc::serde::{Deserialize, Serialize};

pub use microservices_core_types::{OrderResult, OrderResultOut};
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

#[derive(TahiniType, TahiniDeserialize, Debug)]
pub struct SendOrderConfirmationRequest {
    pub email: BBox<String, EmailAddressPolicy>,
    pub order: OrderResult,
}

#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType)]
#[alohomora_out_type(to_derive = [TahiniSerialize, Clone])]
pub struct UsableSendOrderConfirmationRequest {
    pub email: BBox<String, EmailUsablePolicy>,
    pub order: OrderResult,
}

impl TahiniTransformInto<UsableSendOrderConfirmationRequest> for SendOrderConfirmationRequest {
    fn transform_into(
        self,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<UsableSendOrderConfirmationRequest, String> {
        Ok(UsableSendOrderConfirmationRequest {
            email: self.email.transform_into(context)?,
            order: self.order,
        })
    }
}

//==================REIMPLEMENTING FOREIGN TYPES=================

pub use microservices_core_types::Money;

pub use microservices_core_types::Address;

pub use microservices_core_types::CartItem;

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct EmailUsablePolicy;

impl SimplePolicy for EmailUsablePolicy {
    fn simple_name(&self) -> String {
        "EmailUsablepolicy".to_string()
    }

    fn simple_check(
        &self,
        context: &alohomora::context::UnprotectedContext,
        _reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match context.downcast_ref::<String>() {
            None => false,
            Some(context) => context.as_str() == "EmailConfirmation",
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {}
}

impl PolicyInto<EmailUsablePolicy> for EmailAddressPolicy {
    fn into_policy(
        self,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<EmailUsablePolicy, String> {
        match context.service.as_str() {
            "Email" => match context.rpc.as_str() {
                "send_order_confirmation" => Ok(EmailUsablePolicy),
                _ => Err("Illegal transformation".to_string()),
            },
            _ => Err("Illegal transformation".to_string()),
        }
    }
}
