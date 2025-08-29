use alohomora::bbox::{BBox, BBoxRender};
use alohomora::policy::NoPolicy;
use alohomora::pure::PrivacyPureRegion;
use alohomora::rocket::{get, BBoxCookieJar, BBoxTemplate};
use ad_service::types::Ad;
use productcatalog_service::types::Product;

use crate::{
    middleware::{SharedRenderingContext},
    rpcs::{
        ad::get_ad, cart::get_cart, catalog::{get_product}, currency::{convert_currency}, recommendation::list_recommendations
    },
    utils::{cart_size, user_session_id, ProductView},
};

#[derive(BBoxRender)]
struct ProductContext {
    ad: BBox<Option<Ad>, NoPolicy>,
    show_currencies: bool,
    product: ProductView,
    recommendations: Vec<Product>,
    cart_size: BBox<i32, NoPolicy>,
}

#[get("/product/<id>")]
pub async fn product_detail(
    id: BBox<String, NoPolicy>,
    template_context: SharedRenderingContext,
    cookie_jar: BBoxCookieJar<'_, '_>,
) -> BBoxTemplate {
    let t_ctx = tarpc::context::current();
    let current_currency = crate::utils::current_user_currency(&cookie_jar);
    let product = get_product(t_ctx, id.clone()).await;
    let localized_price =
        convert_currency(t_ctx, product.price_usd.clone(), current_currency).await;
    let product_view = ProductView {
        item: product,
        price: localized_price,
    };
    let session_id = user_session_id(&cookie_jar);
    let cart_size = cart_size(&get_cart(t_ctx, session_id.clone()).await.items);

    let recommendations = list_recommendations(
        t_ctx,
        session_id,
        id.into_ppr(PrivacyPureRegion::new(|id: String| vec![id]))
    ).await;

    let ad = get_ad(
        t_ctx,
        product_view.item.categories.clone(),
        BBox::new(00000, NoPolicy {})
    ).await;

    let ad = match ad {
        Some(ad) => ad.into_ppr(PrivacyPureRegion::new(Option::Some)),
        None => BBox::new(None, NoPolicy {}),
    };

    let context = ProductContext {
        ad,
        product: product_view,
        show_currencies: true,
        recommendations,
        cart_size,
    };

    let full_context = template_context.extend_with_handler_context(context);
    BBoxTemplate::render::<_, _, ()>("product.html.tera", &full_context, todo!())
}
