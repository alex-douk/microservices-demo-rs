use alohomora::bbox::{BBox, BBoxRender};
use alohomora::policy::{NoPolicy, Policy};
use alohomora::pure::PrivacyPureRegion;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize, TahiniType};
use alohomora::SesameType;
use serde::{Serialize, Deserialize};

use crate::policies::{AddressPolicy, CreditCardPolicy, MoneyPolicy, CartPolicy};

#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, Clone, BBoxRender])]
pub struct CartItem {
    pub product_id: BBox<String, CartPolicy>,
    pub quantity: BBox<i64, NoPolicy>,
}


#[derive(TahiniSerialize, TahiniDeserialize, TahiniType, SesameType, Debug, Clone, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, Debug, Deserialize, Clone, BBoxRender, TahiniType])]
pub struct Money {
    // The 3-letter currency code defined in ISO 4217.
    pub currency_code: BBox<String, NoPolicy>,
    // The whole units of the amount.
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
impl Money {
    pub fn from(out: BBox<MoneyOut, NoPolicy>) -> Money {
        Money {
            units: out.ppr(PrivacyPureRegion::new(|m: &MoneyOut| m.units)).to_owned_policy(),
            nanos: out.ppr(PrivacyPureRegion::new(|m: &MoneyOut| m.nanos)).to_owned_policy(),
            currency_code: out.into_ppr(PrivacyPureRegion::new(|m: MoneyOut| m.currency_code)),
        }
    }
}

#[derive(TahiniType, Deserialize, Debug, Clone, SesameType, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, BBoxRender, Clone])]
pub struct OrderItem {
    pub item: CartItem,
    pub cost: Money,
}

#[derive(TahiniSerialize, TahiniType, Deserialize, Debug, Clone, Default, SesameType, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, BBoxRender, Clone])]
pub struct Address {
    pub street_address: BBox<String, NoPolicy>,
    pub city: BBox<String, NoPolicy>,
    pub state: BBox<String, NoPolicy>,
    pub country: BBox<String, NoPolicy>,
    pub zip_code: BBox<i64, NoPolicy>
}

#[derive(TahiniType, Deserialize, Debug, Clone, SesameType)]
pub struct CreditCardInfo {
    pub credit_card_number: BBox<String, CreditCardPolicy>,
    pub credit_card_cvv: BBox<i32, CreditCardPolicy>,
    pub credit_card_expiration_year: BBox<i32, CreditCardPolicy>,
    pub credit_card_expiration_month: BBox<i32, CreditCardPolicy>,
}

#[derive(TahiniType, TahiniDeserialize, Debug, Clone, SesameType, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, BBoxRender, Clone])]
pub struct OrderResult {
    pub order_id: BBox<String, NoPolicy>,
    pub shipping_tracking_id: BBox<String, NoPolicy>,
    pub shipping_cost: Money,
    pub shipping_address: Address,
    pub items: Vec<OrderItem>
}

pub mod money;
pub mod policies;
