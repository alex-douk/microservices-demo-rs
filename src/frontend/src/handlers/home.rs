use alohomora::bbox::{BBox, BBoxRender};
use alohomora::policy::NoPolicy;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{get, BBoxCookieJar, BBoxRedirect, BBoxTemplate};
use ad_service::types::Ad;
use currency_service::types::Money;
use productcatalog_service::types::Product;
use crate::{
    middleware::{SharedRenderingContext, COOKIE_CURRENCY},
    rpcs::{
        ad::get_ad, cart::get_cart, catalog::{get_product, list_products}, currency::{convert_currency, get_currencies}
    },
    utils::{self, cart_size, user_session_id, ProductView},
};

#[derive(BBoxRender)]
struct HomeContext {
    show_currency: bool,
    currencies: Vec<String>,
    base_url: String,
    products: Vec<ProductView>,
    cart_size: BBox<i32, NoPolicy>,
    banner_color: String,
    ad: BBox<Option<Ad>, NoPolicy>,
}
///Route for the main page.
///Google's OnlineBoutique implementation supports both GET and HEAD requests.
///Rocket [routing logic](https://rocket.rs/guide/v0.5/requests/#methods) automatically derives
///HEAD request routing if the path supports GET requests.
#[get("/")]
pub async fn home(
    template_context: SharedRenderingContext,
    cookie_jar: BBoxCookieJar<'_, '_>,
) -> BBoxTemplate {
    let t_ctx = tarpc::context::current();
    let currencies = get_currencies(t_ctx).await;

    let current_currency = utils::current_user_currency(&cookie_jar);

    let products = list_products(t_ctx).await;

    let cart = get_cart(t_ctx, user_session_id(&cookie_jar)).await;

    let mut products_localized: Vec<ProductView> = Vec::with_capacity(products.len());

    for product in products.into_iter() {
        let money_usd = Money::from(BBox::new(product.price_usd.clone(), NoPolicy {}));
        let localized_price =
            convert_currency(t_ctx, money_usd, current_currency.clone()).await;
        products_localized.push(ProductView {
            item: Product::from(BBox::new(product, NoPolicy {})),
            price: localized_price,
        });
    }

    let ad = get_ad(
        t_ctx,
        BBox::new(Vec::new(), NoPolicy {}),
        BBox::new(00000, NoPolicy {})
    ).await;

    let ad = match ad {
        Some(ad) => ad.into_ppr(PrivacyPureRegion::new(Option::Some)),
        None => BBox::new(None, NoPolicy {}),
    };

    let context = HomeContext {
        show_currency: true,
        currencies,
        base_url: "".to_string(),
        products: products_localized,
        cart_size: cart_size(&cart.items),
        banner_color: "".to_string(),
        ad
    };

    let full_context = template_context.extend_with_handler_context(context);
    BBoxTemplate::render::<_, _, ()>("home.html.tera", &full_context, todo!())
}


#[get("/checkout")]
pub fn logout(
    cookie_jar: BBoxCookieJar<'_, '_>
) -> BBoxRedirect {
    for cookie in cookie_jar.iter() {
        cookie_jar.remove(cookie_jar.get::<NoPolicy>(cookie).unwrap());
    }
    BBoxRedirect::to2("/")
}
