use ad_service::types::Ad;
use rocket::{
    figment::util, http::{ContentType, CookieJar}, response::{content::RawHtml, Redirect, Responder}, route, uri, Request
};
use rocket_dyn_templates::{Metadata, Template};
use serde_json::Value;

use crate::{
    middleware::{SharedRenderingContext, COOKIE_CURRENCY},
    rpcs::{
        cart::get_cart,
        catalog::{get_product, list_products},
        currency::{convert_currency, get_currencies},
    },
    utils::{self, cart_size, user_session_id, ProductView},
};

#[derive(serde::Serialize)]
struct HomeContext {
    show_currency: bool,
    currencies: Vec<String>,
    base_url: String,
    products: Vec<ProductView>,
    cart_size: i32,
    banner_color: String,
    ad: Ad,
}
///Route for the main page.
///Google's OnlineBoutique implementation supports both GET and HEAD requests.
///Rocket [routing logic](https://rocket.rs/guide/v0.5/requests/#methods) automatically derives
///HEAD request routing if the path supports GET requests.
#[route(GET, uri = "/")]
pub async fn home(
    template_context: SharedRenderingContext,
    cookie_jar: &CookieJar<'_>,
) -> (ContentType, Template) {
    let t_ctx = tarpc::context::current();
    let currencies = get_currencies(t_ctx).await;

    let current_currency = utils::current_user_currency(cookie_jar);

    let products = list_products(t_ctx).await;

    let cart = get_cart(t_ctx, user_session_id(cookie_jar)).await;

    let mut products_localized: Vec<ProductView> = Vec::with_capacity(products.len());

    for product in products.into_iter() {
        let localized_price =
            convert_currency(t_ctx, product.price_usd.clone(), current_currency.clone()).await;
        products_localized.push(ProductView {
            item: product,
            price: localized_price,
        });
    }

    let context = HomeContext {
        show_currency: true,
        currencies,
        base_url: "".to_string(),
        products: products_localized,
        cart_size: cart_size(&cart.items),
        banner_color: "".to_string(),
        ad: Ad {
            redirect_url: "https://youtube.com".to_string(),
            text: "AN INCREDIBLE friend WEBSITE TO VISIT".to_string(),
        },
    };

    let full_context = template_context.extend_with_handler_context(context);
    (
        ContentType::HTML,
        Template::render("home.html.tera", full_context),
    )
}


#[route(GET, uri = "/checkout")]
pub fn logout(
    cookie_jar: &CookieJar<'_>
) -> Redirect {
    for cookie in cookie_jar.iter() {
        cookie_jar.remove(cookie.clone());
    }
    Redirect::found(uri!("/"))
}
