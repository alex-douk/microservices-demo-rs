use alohomora::bbox::{BBox, BBoxRender};
use alohomora::policy::NoPolicy;
use alohomora::pure::PrivacyPureRegion;
use alohomora::SesameType;
use microservices_core_types::policies::CartPolicy;
use tahini_tarpc::{TahiniTransformInto, TahiniType};
use tarpc::serde::{Serialize, Deserialize};


#[derive(TahiniType, Deserialize, Debug, Clone, SesameType, BBoxRender)]
#[alohomora_out_type(to_derive = [Serialize, Deserialize, Debug, Clone, TahiniType])]
pub struct Product {
    pub id: BBox<String, CartPolicy>,
    pub name: BBox<String, CartPolicy>,
    pub description: BBox<String, CartPolicy>,
    pub picture: BBox<String, CartPolicy>,
    pub price_usd: Money,
    pub categories : BBox<Vec<String>, CartPolicy>,
}

impl Product {
    pub fn from(out: BBox<ProductOut, CartPolicy>) -> Product {
        Product {
            id: out.clone().into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.id)),
            name: out.clone().into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.name)),
            description: out.clone().into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.description)),
            picture: out.clone().into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.picture)),
            price_usd: Money::from(out.clone().into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.price_usd))),
            categories: out.into_ppr(PrivacyPureRegion::new(|p: ProductOut| p.categories)),
        }
    }
}


#[derive(TahiniType, Serialize, Deserialize, Debug, Clone)]
pub struct ListProductResponse {
    pub products: Vec<ProductOut>
}

#[derive(TahiniType, Deserialize, Debug, Clone)]
pub struct GetProductRequest {
    pub id: BBox<String, CartPolicy>,
}

impl TahiniTransformInto<GetProductRequest> for GetProductRequest {
    fn transform_into(self, context: &tahini_tarpc::context::TahiniContext) -> Result<GetProductRequest, String> {
        Ok(self)
    }
}

#[derive(TahiniType, Serialize, Deserialize, Debug, Clone)]
pub struct SearchProductRequest {
    pub query: BBox<String, NoPolicy>,
}


#[derive(TahiniType, Serialize, Deserialize, Debug, Clone)]
pub struct SearchProductResponse {
    pub results: BBox<Vec<ProductOut>, NoPolicy>
}
//==================REIMPLEMENTING FOREIGN TYPES=================


pub use microservices_core_types::Money;
use microservices_core_types::MoneyOut;
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
