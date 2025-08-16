use std::{collections::HashMap, convert::Infallible};

use chrono::{Datelike, NaiveDate};
use rocket::{
    fairing::{Fairing, Kind},
    http::{Cookie, Status},
    outcome::Outcome,
    request::FromRequest,
    time::macros::utc_datetime,
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

    async fn on_request(&self, req: &mut rocket::Request<'_>, data: &mut rocket::Data<'_>) {
        let cookie_jar = req.cookies();
        if let None = cookie_jar.get(COOKIE_SESSION_ID) {
            //Bound to session
            cookie_jar
                .add(Cookie::build((COOKIE_SESSION_ID, Uuid::new_v4().to_string())).expires(None));
        }
    }
}

pub struct RequestId(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestId {
    type Error = Infallible;

    async fn from_request(
        _request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        Outcome::Success(RequestId(Uuid::new_v4().to_string()))
    }
}


#[derive(Serialize, Clone)]
pub struct SharedRenderingContext {
    session_id: Option<String>,
    request_id: String,
    user_currency: String,
    platform_css: String,
    platform_name: String,
    is_cymbal_brand: bool,
    assistant_enabled: bool,
    //We purposefully do not fill deployment details
    frontend_message: String,
    current_year: i32,
    base_url: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SharedRenderingContext {
    type Error = Infallible;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let cookie_jar = request.cookies();
        let session_id = cookie_jar.get(COOKIE_SESSION_ID);

        let request_id = request
            .guard::<RequestId>()
            .await
            .expect("RequestId should never fail");
        let currency = match cookie_jar.get(COOKIE_CURRENCY) {
            None => DEFAULT_CURRENCY.to_string(),
            Some(currency_cookie) => currency_cookie.value().to_string(),
        };

        //We have a static baseUrl
        let base_url = "".to_string();
        let context = SharedRenderingContext {
            session_id: session_id.map(|c| c.value().to_string()),
            request_id: request_id.0,
            user_currency: currency,
            platform_css: "aws".to_string(),
            platform_name: "aws".to_string(),
            is_cymbal_brand: false,
            //No LLM here
            assistant_enabled: false,
            frontend_message: "Welcome to my awesome site".to_string(),
            current_year: chrono::Utc::now().year(),
            base_url,
        };
        Outcome::Success(context)
    }
}

impl SharedRenderingContext {
    pub fn extend_with_handler_context<T: Serialize>(self, local_context: T) -> Map<String, Value> {
        let local_context_val =
            serde_json::to_value(local_context).expect("Couldn't serialize the local context");

        if let Value::Object(map) = local_context_val {
            let mut total_context = self.build_self_map_with_added_capacity(map.len());
            // let map = Map::from_iter(local_context.into_iter());
            total_context.extend(map.into_iter());
            return total_context;
        }
        unreachable!("Local rendering context was not a JSON map")
    }

    fn build_self_map_with_added_capacity(self, bonus_capacity: usize) -> Map<String, Value> {
        let mut total_context = Map::with_capacity(10 + bonus_capacity);
        if let Some(session_id) = self.session_id {
            total_context.insert("session_id".to_string(), Value::String(session_id));
        }
        total_context.insert("request_id".to_string(), Value::String(self.request_id));
        total_context.insert(
            "user_currency".to_string(),
            Value::String(self.user_currency),
        );
        total_context.insert("platform_css".to_string(), Value::String(self.platform_css));
        total_context.insert(
            "platform_name".to_string(),
            Value::String(self.platform_name),
        );
        total_context.insert(
            "is_cymbal_brand".to_string(),
            Value::Bool(self.is_cymbal_brand),
        );
        total_context.insert(
            "assistant_enabled".to_string(),
            Value::Bool(self.assistant_enabled),
        );
        total_context.insert(
            "frontend_message".to_string(),
            Value::String(self.frontend_message),
        );
        total_context.insert(
            "current_year".to_string(),
            Value::Number(self.current_year.into()),
        );
        total_context.insert("base_url".to_string(), Value::String(self.base_url));
        total_context
    }
}
