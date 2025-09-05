use alohomora::policy::{FrontendPolicy, SimplePolicy};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};
use alohomora::policy::Reason;

#[derive(TahiniSerialize, TahiniDeserialize, Clone)]
pub struct EmailAddressPolicy;

impl SimplePolicy for EmailAddressPolicy {
    fn simple_name(&self) -> String {
        "EmailAddressPolicy".to_string()
    }

    fn simple_check(&self, _context: &alohomora::context::UnprotectedContext, reason: alohomora::policy::Reason<'_>) -> bool {
        match reason {
            Reason::TemplateRender("confirmation") => true,
            Reason::Response => true,
            _ => false
        }
    }

    fn simple_join_direct(&mut self, _other: &mut Self) {
    }
}

impl FrontendPolicy for EmailAddressPolicy {
    fn from_cookie<'a, 'r>(
            name: &str,
            cookie: &'a rocket::http::Cookie<'static>,
            request: &'a rocket::Request<'r>,
        ) -> Self
        where
            Self: Sized {
        Self
    }

    fn from_request<'a, 'r>(request: &'a rocket::Request<'r>) -> Self
        where
            Self: Sized {
        Self
    }
}
