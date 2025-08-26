use std::collections::HashMap;

use alohomora::policy::{Policy, PolicyAnd, Reason};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

//Needs to be provided by the checkout service,
//which transforms into the payment service's.
//Otherwise we end up having some storage-specific leakage end-to-end
#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct CreditCardPolicy {
    third_party_vendors: HashMap<String, bool>,
    store_payment_info: bool,
}

impl Policy for CreditCardPolicy {
    fn name(&self) -> String {
        "CreditCardPolicy".to_string()
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<Self>() {
            let spec_other = other.specialize::<Self>().unwrap();
            self.join_logic(spec_other).map(|a| a.into_any())
        } else {
            Ok(PolicyAnd::new(self.clone(), other).into_any())
        }
    }

    fn join_logic(&self, other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let mut map = other.third_party_vendors;
        for (k, v) in self.third_party_vendors.iter() {
            match map.get_mut(k) {
                None => {
                    map.insert(k.clone(), *v);
                }
                Some(t) => *t = *t && *v,
            }
        }
        Ok(Self {
            third_party_vendors: map,
            store_payment_info: self.store_payment_info && other.store_payment_info,
        })
    }

    //Under the shared CC policy, no one should have access to it except the user
    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        if let Reason::Response = reason {
            true
        } else {
            false
        }
    }
}
