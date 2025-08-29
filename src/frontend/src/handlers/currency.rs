use std::convert::Infallible;
use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use alohomora::rocket::{post, BBoxCookie, BBoxCookieJar, BBoxForm, BBoxRedirect, BBoxRequest, BBoxRequestOutcome, FromBBoxForm, FromBBoxRequest};

use crate::middleware::COOKIE_CURRENCY;

#[derive(FromBBoxForm)]
pub struct CurrencyForm {
    currency_code: BBox<String, NoPolicy>,
}

pub struct RedirectHeader(pub Option<String>);

#[rocket::async_trait]
impl<'a, 'r> FromBBoxRequest<'a, 'r> for RedirectHeader {
    type BBoxError = Infallible;

    async fn from_bbox_request(request: BBoxRequest<'a, 'r>) -> BBoxRequestOutcome<Self, Self::BBoxError> {
        let a = request.headers().get_one::<NoPolicy>("referer");
        BBoxRequestOutcome::Success(RedirectHeader(a.map(|e| e.discard_box())))
    }
}

#[post("/", data = "<currency_form>")]
pub fn set_currency(
    currency_form: BBoxForm<CurrencyForm>,
    cookie_jar: BBoxCookieJar<'_, '_>,
    referer: RedirectHeader,
) -> BBoxRedirect {
    // match cookie_jar.get(COOKIE_CURRENCY) {
    //     //TODO: Update expiration time on cookie
    //     None => cookie_jar.add(Cookie::build((COOKIE_CURRENCY, currency_form.currency_code)).expires(None)),
    //     Some(cookie) => cookie.set_value(currency_form.currency_code),
    // }
    cookie_jar.add::<_, ()>(
            BBoxCookie::build(
                COOKIE_CURRENCY,
                currency_form.into_inner().currency_code
            ).finish(),
        todo!(),
    );

    let location = match referer.0 {
        None => "/".to_string(),
        Some(referer) => referer,
    };
    BBoxRedirect::to2(location)
}
