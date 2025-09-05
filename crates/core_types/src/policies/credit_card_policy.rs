use alohomora::policy::FrontendPolicy;
use alohomora::policy::Reason;
use alohomora::policy::SimplePolicy;
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use rocket;


#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct CreditCardPolicy {
    pub third_party_vendors: Vec<String>,
    pub store_payment_info: bool,
}

impl SimplePolicy for CreditCardPolicy {
    fn simple_name(&self) -> String {
        "CreditCardPolicy".to_string()
    }

    fn simple_check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        match reason {
            Reason::Response => true,
            _ => false,
        }
    }

    fn simple_join_direct(&mut self, other: &mut Self) {
        self.third_party_vendors
            .retain(|v| other.third_party_vendors.contains(v));
    }
}

impl FrontendPolicy for CreditCardPolicy {
    fn from_request<'a, 'r>(request: &'a rocket::Request<'r>) -> Self
        where
            Self: Sized {
                Self {
                    third_party_vendors: Vec::new(),
                    store_payment_info: false
                }
    }

    fn from_cookie<'a, 'r>(
            name: &str,
            cookie: &'a rocket::http::Cookie<'static>,
            request: &'a rocket::Request<'r>,
        ) -> Self
        where
            Self: Sized {
                Self::from_request(request)
    }
}
