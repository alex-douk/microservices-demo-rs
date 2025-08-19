use ad_service::types::Ad;
use currency_service::types::Money;
use productcatalog_service::types::Product;
use rocket::{
    http::{ContentType, CookieJar},
    response::{content::RawHtml, Responder},
    route, Request,
};
use rocket_dyn_templates::{Metadata, Template};
use serde_json::Value;

use crate::{
    middleware::{SharedRenderingContext, COOKIE_CURRENCY},
    rpcs::{
        ad::get_ad, cart::get_cart, catalog::{get_product, list_products}, currency::{convert_currency, get_currencies}, recommendation::list_recommendations
    },
    utils::{cart_size, user_session_id, ProductView},
};

#[derive(serde::Serialize)]
struct ProductContext {
    ad: Option<Ad>,
    show_currencies: bool,
    product: ProductView,
    recommendations: Vec<Product>,
    cart_size: i32,
}

#[route(GET, uri = "/product/<id>")]
pub async fn product_detail(
    id: String,
    template_context: SharedRenderingContext,
    cookie_jar: &CookieJar<'_>,
) -> (ContentType, Template) {
    let t_ctx = tarpc::context::current();
    let current_currency = crate::utils::current_user_currency(cookie_jar);
    let product = get_product(t_ctx, id.clone()).await;
    let localized_price =
        convert_currency(t_ctx, product.price_usd.clone(), current_currency).await;
    let product_view = ProductView {
        item: product,
        price: localized_price,
    };
    let session_id = user_session_id(cookie_jar);
    let cart_size = cart_size(&get_cart(t_ctx, session_id.clone()).await.items);

    let recommendations = list_recommendations(t_ctx, session_id, vec![id]).await;


    let context = ProductContext {

        ad: get_ad(t_ctx, product_view.item.categories.clone(), 00000).await,
        product: product_view,
        show_currencies: true,
        recommendations,
        cart_size,
    };

    let full_context = template_context.extend_with_handler_context(context);
    (
        ContentType::HTML,
        Template::render("product.html.tera", full_context),
    )
}
