use std::convert::Infallible;
use alohomora::bbox::{BBox, BBoxRender, Renderable};
use alohomora::policy::NoPolicy;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{BBoxRequest, BBoxRequestOutcome, FromBBoxRequest};
use chrono::{Datelike};
use rocket::{
    fairing::{Fairing, Kind},
    http::{Cookie},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
// use rocket_dyn_templates::{Engines, Template};
// use serde::Serialize;
// use serde_json::{Map, Value};
use uuid::Uuid;

pub struct EnsureSessionId;

pub static DEFAULT_CURRENCY: &'static str = "USD";
pub static COOKIE_PREFIX: &'static str = "shop_";
pub static COOKIE_SESSION_ID: &'static str = "shop_session-id";
pub static COOKIE_CURRENCY: &'static str = "shop_currency";

#[rocket::async_trait]
impl Fairing for EnsureSessionId {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Ensuring sessionID is bound",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        let cookie_jar = req.cookies();
        if let None = cookie_jar.get(COOKIE_SESSION_ID) {
            //Bound to session
            cookie_jar.add(
                Cookie::build(COOKIE_SESSION_ID, Uuid::new_v4().to_string()).finish()
            );
        }
    }
}

pub struct RequestId(pub BBox<String, NoPolicy>);

#[rocket::async_trait]
impl<'a, 'r> FromBBoxRequest<'a, 'r> for RequestId {
    type BBoxError = Infallible;

    async fn from_bbox_request(request: BBoxRequest<'a, 'r>) -> BBoxRequestOutcome<Self, Self::BBoxError> {
        let uuid = Uuid::new_v4().to_string();
        let policy = NoPolicy {};
        BBoxRequestOutcome::Success(RequestId(BBox::new(uuid, policy)))
    }
}


#[derive(BBoxRender, Clone)]
pub struct SharedRenderingContext {
    session_id: BBox<Option<String>, NoPolicy>,
    request_id: BBox<String, NoPolicy>,
    user_currency: BBox<String, NoPolicy>,
    platform_css: String,
    platform_name: String,
    is_cymbal_brand: bool,
    assistant_enabled: bool,
    // We purposefully do not fill deployment details
    frontend_message: String,
    current_year: BBox<i32, NoPolicy>,
    base_url: String,
}

#[rocket::async_trait]
impl<'a, 'r> FromBBoxRequest<'a, 'r> for SharedRenderingContext {
    type BBoxError = Infallible;

    async fn from_bbox_request(request: BBoxRequest<'a, 'r>) -> BBoxRequestOutcome<Self, Self::BBoxError> {
        let cookie_jar = request.cookies();
        let session_id = match cookie_jar.get(COOKIE_SESSION_ID) {
            None => BBox::new(None, NoPolicy {}),
            Some(cookie) => cookie.value().to_owned().into_ppr(PrivacyPureRegion::new(Option::Some)),
        };

        let request_id = request
            .guard::<RequestId>()
            .await
            .expect("RequestId should never fail");
        let currency = match cookie_jar.get(COOKIE_CURRENCY) {
            None => BBox::new(DEFAULT_CURRENCY.to_string(), NoPolicy {}),
            Some(currency_cookie) => currency_cookie.value().to_owned(),
        };

        //We have a static baseUrl
        let base_url = "".to_string();
        let context = SharedRenderingContext {
            session_id,
            request_id: request_id.0,
            user_currency: currency,
            platform_css: "aws".to_string(),
            platform_name: "aws".to_string(),
            is_cymbal_brand: false,
            //No LLM here
            assistant_enabled: false,
            frontend_message: "Welcome to my awesome site".to_string(),
            current_year: BBox::new(chrono::Utc::now().year(), NoPolicy {}),
            base_url,
        };
        BBoxRequestOutcome::Success(context)
    }
}

pub struct MyRender<T: BBoxRender>(
    pub SharedRenderingContext,
    pub T,
);
impl<T: BBoxRender> BBoxRender for MyRender<T> {
    fn render(&self) -> Renderable {
        let mut this_map = if let Renderable::Dict(map) = self.1.render() {
            map
        } else {
            unreachable!("Self rendering context was not a JSON map");
        };

        let local_context_map = if let Renderable::Dict(map) = self.1.render() {
            map
        } else {
            unreachable!("Local rendering context was not a JSON map");
        };

        this_map.extend(local_context_map);
        Renderable::Dict(this_map)
    }
}

impl SharedRenderingContext {
    pub fn extend_with_handler_context<T: BBoxRender>(self, local_context: T) -> MyRender<T> {
        MyRender(self, local_context)
    }
}
