use alohomora::bbox::{BBox, BBoxRender};
use alohomora::context::Context;
use alohomora::policy::NoPolicy;
use checkout_service::types::{Address, CreditCardInfo, OrderResult};
use chrono::Datelike;
use currency_service::{money::sum, types::Money};
use productcatalog_service::types::Product;
use microservices_core_types::policies::{CreditCardPolicy, EmailAddressPolicy};

use alohomora::rocket::{get, post};
use alohomora::rocket::{BBoxForm, BBoxRedirect, FromBBoxForm, BBoxTemplate, BBoxCookieJar};

use either::Either;

use crate::{
    middleware::SharedRenderingContext,
    rpcs::{
        cart::{add_item, delete_cart, get_cart},
        catalog::get_product,
        checkout::checkout as rpc_checkout,
        currency::{convert_currency, get_currencies},
        recommendation::list_recommendations,
        shipping::get_quote,
    },
    utils::{cart_size, current_user_currency, user_session_id},
};

#[derive(BBoxRender)]
struct CartItemView {
    item: Product,
    quantity: BBox<i64, NoPolicy>,
    price: Money,
}

#[derive(BBoxRender)]
struct ViewCartContext {
    currencies: Vec<String>,
    recommendations: Vec<Product>,
    cart_size: BBox<i32, NoPolicy>,
    shipping_cost: Money,
    show_currency: bool,
    total_cost: Money,
    items: Vec<CartItemView>,
    expiration_years: Vec<BBox<i32, NoPolicy>>,
}



#[get("/")]
pub async fn view_cart(
    template_context: SharedRenderingContext,
    cookie_jar: BBoxCookieJar<'_, '_>,
) -> BBoxTemplate {
    let t_ctx = tarpc::context::current();
    let supported_currencies = get_currencies(t_ctx).await;
    let current_user_currency = current_user_currency(&cookie_jar);
    let session_id = user_session_id(&cookie_jar);
    let cart = get_cart(t_ctx, session_id.clone()).await;

    let cart_size = cart_size(&cart.items);

    // let mut products = Vec::with_capacity(cart.items.len());
    let item_ids = cart
        .items
        .iter()
        .map(|item| &item.product_id)
        .cloned()
        .collect::<Vec<_>>();

    let item_ids: BBox<Vec<_>, _> = item_ids.into();
    let item_ids = match item_ids.specialize_option_policy() {
        Either::Left(no_policy) => {
            // Vector is empty and there is no policy.
            // Create one.
            BBox::new(Vec::new(), NoPolicy {})
        },
        Either::Right(policy_exists) => policy_exists,
    };

    // for id in item_ids {
    //     products.push(get_product(t_ctx, id).await);
    // }
    //
    let mut cart_item_views: Vec<CartItemView> = Vec::with_capacity(cart.items.len());

    let recommendations = list_recommendations(t_ctx, session_id, item_ids).await;
    let shipping_cost = get_quote(
        t_ctx,
        Address::default(),
        cart.items.clone(),
        current_user_currency.clone(),
    )
    .await;

    let mut total_price = Money {
        currency_code: current_user_currency.clone(),
        units: BBox::new(0, NoPolicy {}),
        nanos: BBox::new(0, NoPolicy {}),
    };

    for item in cart.items {
        let prod = get_product(t_ctx, item.product_id).await;
        let localized_price =
            convert_currency(t_ctx, prod.price_usd.clone(), current_user_currency.clone()).await;
        let total_article_price =
            currency_service::money::slow_multiply(&localized_price, item.quantity.clone());

        //TODO: Replace sum and multiply as a method of a mutable price instead of floating
        //functions or at least operate on mutable references to not have to clone and move everything
        //single time....
        total_price = currency_service::money::sum(&total_price, &total_article_price)
            .expect("Couldn't sum article price to total price");
        cart_item_views.push(CartItemView {
            item: prod,
            quantity: item.quantity,
            price: total_article_price,
        });
    }

    total_price =
        sum(&total_price, &shipping_cost).expect("Couldn't sum shipping and article clost");
    let current_year = chrono::Utc::now().year();

    let local_context = ViewCartContext {
        currencies: supported_currencies,
        recommendations,
        cart_size,
        shipping_cost,
        show_currency: true,
        total_cost: total_price,
        items: cart_item_views,
        expiration_years: (0..=4).map(|i| BBox::new(current_year + i, NoPolicy {})).collect(),
    };

    let total_context = template_context.extend_with_handler_context(local_context);
    BBoxTemplate::render::<_, _, ()>("cart.html.tera", &total_context, todo!())
}

#[derive(FromBBoxForm)]
pub struct AddToCartForm {
    quantity: BBox<i64, NoPolicy>,
    product_id: BBox<String, NoPolicy>,
}

#[post("/", data = "<cart_form>")]
pub async fn add_to_cart(cookie_jar: BBoxCookieJar<'_, '_>, cart_form: BBoxForm<AddToCartForm>) -> BBoxRedirect {
    println!("GETTING TO ADD");
    //TODO: Validate the form
    let t_ctx = tarpc::context::current();
    let product = get_product(t_ctx, cart_form.product_id.clone()).await;
    let session_id = user_session_id(&cookie_jar);
    add_item(t_ctx, session_id, product.id, cart_form.into_inner().quantity).await;
    BBoxRedirect::to2("/cart")
}

#[post("/empty")]
pub async fn empty_cart(cookie_jar: BBoxCookieJar<'_, '_>) -> BBoxRedirect {
    let t_ctx = tarpc::context::current();
    let session_id = user_session_id(&cookie_jar);
    delete_cart(t_ctx, session_id).await;
    BBoxRedirect::to2("/")
}

#[derive(FromBBoxForm)]
pub struct CheckoutForm {
    email: BBox<String, EmailAddressPolicy>,
    street_address: BBox<String, NoPolicy>,
    zip_code: BBox<i64, NoPolicy>,
    city: BBox<String, NoPolicy>,
    state: BBox<String, NoPolicy>,
    country: BBox<String, NoPolicy>,
    credit_card_number: BBox<String, CreditCardPolicy>,
    credit_card_expiration_month: BBox<i32, CreditCardPolicy>,
    credit_card_expiration_year: BBox<i32, CreditCardPolicy>,
    credit_card_cvv: BBox<i32, CreditCardPolicy>,
    store_payment_info: bool
}

#[derive(BBoxRender)]
struct OrderRenderingContext{
    show_currency: bool,
    currencies: Vec<String>,
    order: OrderResult,
    total_paid: Money,
    recommendations: Vec<Product>
}

#[post("/checkout", data = "<checkout_form>")]
pub async fn checkout(
    checkout_form: BBoxForm<CheckoutForm>,
    cookie_jar: BBoxCookieJar<'_, '_>,
    template_context: SharedRenderingContext
) -> BBoxTemplate {
    let user_currency = current_user_currency(&cookie_jar);
    let session_id = user_session_id(&cookie_jar);
    let t_ctx = tarpc::context::current();
    let cc = CreditCardInfo {
        credit_card_number: checkout_form.credit_card_number.clone(),
        credit_card_expiration_month: checkout_form.credit_card_expiration_month.clone(),
        credit_card_expiration_year: checkout_form.credit_card_expiration_year.clone(),
        credit_card_cvv: checkout_form.credit_card_cvv.clone(),
    };

    let address = Address {
        street_address: checkout_form.street_address.clone(),
        city: checkout_form.city.clone(),
        state: checkout_form.state.clone(),
        country: checkout_form.country.clone(),
        zip_code: checkout_form.zip_code.clone(),
    };

    let order = rpc_checkout(
        t_ctx,
        address,
        checkout_form.email.clone(),
        cc,
        session_id.clone(),
        user_currency.clone(),
        checkout_form.store_payment_info
    )
    .await;

    let recommendations = list_recommendations(t_ctx, session_id.clone(), BBox::new(Vec::new(), NoPolicy {})).await;

    let items = order.items.clone();
    let total_price = items
        .iter()
        .map(|item| currency_service::money::slow_multiply(&item.cost, item.item.quantity.clone()))
        .fold(order.shipping_cost.clone(), |acc, e| sum(&acc, &e).expect("Couldn't sum"));

    let currencies = get_currencies(t_ctx).await;


    let local_context  = OrderRenderingContext {
        show_currency: false,
        currencies,
        order,
        total_paid: total_price,
        recommendations
    };

    let total_context = template_context.extend_with_handler_context(local_context);
    BBoxTemplate::render::<_, _, ()>("order.html.tera", &total_context, Context::empty())
}
