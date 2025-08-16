use std::{any::Any, collections::HashMap, env::current_exe};

use currency_service::types::Money;
use rocket::{
    fairing::{Fairing, Kind},
    http::Cookie, response::Responder,
};
use rocket_dyn_templates::{
    tera::{self, Result, Tera},
    Engines, Template,
};
use serde_json::{Map, Number, Value};

pub mod home;
pub mod product;
pub mod currency;
pub mod cart;

pub fn template_fairing() -> impl Fairing {
    let mut template_fairing = Template::custom(configure_engine_helper);
    template_fairing
}

// fn render_currency_logo()
//
fn render_currency_logo(currency_code: &str) -> &'static str {
    match currency_code {
        "USD" => "$",
        "CAD" => "$",
        "JPY" => "¥",
        "EUR" => "€",
        "TRY" => "₺",
        "GBP" => "£",
        _ => "$",
    }
}

fn render_money_inner(currency_code: String, units: i64, nanos: i32) -> String {
    format!(
        "{}{}.{:02}",
        render_currency_logo(currency_code.as_str()),
        units,
        nanos / 10_000_000
    )
}

//
fn configure_engine_helper(engines: &mut Engines) {
    engines.tera = Tera::new("templates/*.html.tera").expect("Couldn't load templates directory");
    //Forced to macro because we borrow the hashmap for the closure, and the return type might not
    //match...
    macro_rules! match_to_err {
        ($T:expr) => {{
            let arg = $T;
            if let None = arg {
                return Err(tera::Error::msg("Item price malformed"));
            }
            arg.unwrap()
        }};
    }
    engines.tera.register_function(
        "render_money",
        Box::new(
            move |args: &HashMap<String, rocket_dyn_templates::tera::Value>| -> Result<Value> {
                let price = match_to_err!(args.get("product"));

                let currency_code_val = match_to_err!(price.get("currency_code"));
                let units_val = match_to_err!(price.get("units"));
                let nanos_val = match_to_err!(price.get("nanos"));

                match (currency_code_val, units_val, nanos_val) {
                    (
                        Value::String(currency_code),
                        Value::Number(units_i64),
                        Value::Number(nanos_i32),
                    ) => Ok(Value::String(render_money_inner(
                        currency_code.to_string(),
                        match_to_err!(units_i64.as_i64()),
                        match_to_err!(nanos_i32.as_i64()) as i32,
                    ))),
                    _ => Err(tera::Error::msg("Product Price Malformed")),
                }
            },
        ),
    );

    engines.tera.register_function(
        "render_currency_logo",
        Box::new(
            move |args: &HashMap<String, tera::Value>| -> Result<Value> {
                let arg = args.get("currency");
                let currency_arg = match arg {
                    None => "",
                    Some(val) => match val {
                        Value::String(currency) => currency.as_str(),
                        _ => "",
                    },
                };
                Ok(Value::String(
                    render_currency_logo(currency_arg).to_string(),
                ))
            },
        ),
    );
}


pub struct Prout(pub Template);

impl<'r, 'o: 'r> Responder<'r, 'o> for Prout {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        println!("CALLING INTO TEMPLATE RESPOND TO");
        self.0.respond_to(request)
    }
}
