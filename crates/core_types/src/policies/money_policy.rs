use alohomora::policy::SimplePolicy;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use alohomora::policy::Reason;

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct MoneyPolicy;

impl SimplePolicy for MoneyPolicy {
    fn simple_name(&self) -> String {
        "MoneyPolicy".to_string()
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
