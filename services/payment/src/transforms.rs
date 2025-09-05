// use tahini_tarpc::TahiniTransformInto;
//
// use crate::types::{ChargeRequest, PaymentChargeRequest};
//
// impl TahiniTransformInto<PaymentChargeRequest> for ChargeRequest {
//     fn transform_into(self, context: &tahini_tarpc::context::TahiniContext) -> Result<PaymentChargeRequest, String> {
//         PaymentChargeRequest {
//             amount: self.amount
//
//         }
//     }
// }
