use alohomora::policy::SimplePolicy;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use alohomora::policy::Reason;

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct CartPolicy;

impl SimplePolicy for CartPolicy {
    fn simple_name(&self) -> String {
        "CartPolicy".to_string()
    }

    fn simple_check(&self, _context: &alohomora::context::UnprotectedContext, reason: alohomora::policy::Reason<'_>) -> bool {
        match reason {
            Reason::Response => true,
            _ => false
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {
    }
}
