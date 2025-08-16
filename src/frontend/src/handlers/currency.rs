use std::convert::Infallible;

use rocket::form::Form;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    route, uri, FromForm,
};

use crate::middleware::COOKIE_CURRENCY;

#[derive(FromForm)]
pub struct CurrencyForm {
    currency_code: String,
}

pub struct RedirectHeader(pub Option<String>);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RedirectHeader {
    type Error = Infallible;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let a = request.headers().get("referer").next();
        rocket::outcome::Outcome::Success(RedirectHeader(a.map(|e| e.to_string())))
    }
}

#[route(POST, uri = "/", data = "<currency_form>")]
pub fn set_currency(
    currency_form: Form<CurrencyForm>,
    cookie_jar: &CookieJar<'_>,
    referer: RedirectHeader,
) -> Redirect {
    // match cookie_jar.get(COOKIE_CURRENCY) {
    //     //TODO: Update expiration time on cookie
    //     None => cookie_jar.add(Cookie::build((COOKIE_CURRENCY, currency_form.currency_code)).expires(None)),
    //     Some(cookie) => cookie.set_value(currency_form.currency_code),
    // }
    cookie_jar
        .add(Cookie::build((COOKIE_CURRENCY, currency_form.currency_code.clone())).expires(None));

    let location = match referer.0 {
        None => "/".to_string(),
        Some(referer) => referer,
    };
    Redirect::found(location)
}
