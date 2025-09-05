use alohomora::policy::SimplePolicy;
use microservices_core_types::{policies::{CreditCardPolicy, MoneyPolicy}};
use tahini_tarpc::{
    traits::PolicyInto, TahiniDeserialize, TahiniSerialize, TahiniType,
};

#[derive(Clone, TahiniSerialize, TahiniDeserialize, TahiniType)]
pub struct PaymentProcessingPolicy {
    pub authorized_payment_processors: Vec<String>,
    pub store_payment_info: bool,
}

pub enum PaymentProcessor {
    Stripe,
    PayPal,
}

impl PaymentProcessor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PayPal => "PayPal",
            Self::Stripe => "Stripe",
        }
    }
}

impl SimplePolicy for PaymentProcessingPolicy {
    fn simple_name(&self) -> String {
        "PaymentProcessingPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            alohomora::policy::Reason::DB(_, _) => self.store_payment_info,
            alohomora::policy::Reason::Custom(boxed_processor) => {
                match boxed_processor.downcast_ref::<PaymentProcessor>() {
                    None => false,
                    Some(proc) => self
                        .authorized_payment_processors
                        .contains(&proc.as_str().to_string()),
                }
            }
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.authorized_payment_processors
            .retain(|p| other.authorized_payment_processors.contains(p));
    }
}

//The payment processing service provides itself the policy transformation from the global policy
//to authenticate itself: if it wants access to the credit card, it needs to do the work for it.
//Clients review the transformation and the policies, and acknowledge it sounds fair.
impl PolicyInto<PaymentProcessingPolicy> for CreditCardPolicy {
    fn into_policy(
        self,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<PaymentProcessingPolicy, String> {
        match context.service.as_str() {
            "Payment" => match context.rpc.as_str() {
                "charge" => Ok(PaymentProcessingPolicy {
                    authorized_payment_processors: self.third_party_vendors,
                    store_payment_info: self.store_payment_info,
                }),
                _ => Err("Wrong RPC on the payment service".to_string()),
            },
            _ => Err("Illegal policy transformation for this service".to_string()),
        }
    }
}


#[derive(Clone, TahiniSerialize, TahiniDeserialize, TahiniType)]
pub struct MoneyProcessingPolicy;

impl SimplePolicy for MoneyProcessingPolicy {
    fn simple_name(&self) -> String {
        "MoneyProcessingPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            alohomora::policy::Reason::DB(_, _) => true,
            alohomora::policy::Reason::Custom(boxed_processor) => {
                match boxed_processor.downcast_ref::<PaymentProcessor>() {
                    None => false,
                    Some(_) => true //The money is oblivious to the processor
                }
            }
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {
    }
}




impl PolicyInto<MoneyProcessingPolicy> for MoneyPolicy {
    fn into_policy(
        self,
        context: &tahini_tarpc::context::TahiniContext,
    ) -> Result<MoneyProcessingPolicy, String> {
        match context.service.as_str() {
            "Payment" => match context.rpc.as_str() {
                "charge" => Ok(MoneyProcessingPolicy),
                _ => Err("Wrong RPC on the payment service".to_string()),
            },
            _ => Err("Illegal policy transformation for this service".to_string()),
        }
    }


}
