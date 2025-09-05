use alohomora::policy::SimplePolicy;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use alohomora::policy::Reason;

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct AddressPolicy{
    targeted_ads_consent: bool
}

impl Default for AddressPolicy {
    fn default() -> Self {
        Self {
            targeted_ads_consent: false
        }
    }
}

impl SimplePolicy for AddressPolicy {
    fn simple_name(&self) -> String {
        "AddressPolicy".to_string()
    }

    fn simple_check(&self, _context: &alohomora::context::UnprotectedContext, reason: alohomora::policy::Reason<'_>) -> bool {
        match reason {
            Reason::Response => true,
            Reason::TemplateRender(template_name) =>  template_name.contains("order") || template_name.contains("confirmation"),
            _ => false
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.targeted_ads_consent = self.targeted_ads_consent && other.targeted_ads_consent
    }
}
