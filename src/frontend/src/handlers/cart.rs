use checkout_service::types::{Address, CreditCardInfo, OrderResult};
use chrono::Datelike;
use currency_service::{money::sum, types::Money};
use productcatalog_service::types::Product;
use rocket::{
    form::Form,
    http::{ContentType, CookieJar},
    response::Redirect,
    route, uri, FromForm,
};
use rocket_dyn_templates::Template;

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

#[derive(serde::Serialize)]
struct CartItemView {
    item: Product,
    quantity: i32,
    price: Money,
}

#[derive(serde::Serialize)]
struct ViewCartContext {
    currencies: Vec<String>,
    recommendations: Vec<Product>,
    cart_size: i32,
    shipping_cost: Money,
    show_currency: bool,
    total_cost: Money,
    items: Vec<CartItemView>,
    expiration_years: Vec<i32>,
}

#[route(GET, uri = "/")]
pub async fn view_cart(
    template_context: SharedRenderingContext,
    cookie_jar: &CookieJar<'_>,
) -> (ContentType, Template) {
    let t_ctx = tarpc::context::current();
    let supported_currencies = get_currencies(t_ctx).await;
    let current_user_currency = current_user_currency(cookie_jar);
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
        units: 0,
        nanos: 0,
    };

    for item in cart.items {
        let prod = get_product(t_ctx, item.product_id).await;
        let localized_price =
            convert_currency(t_ctx, prod.price_usd.clone(), current_user_currency.clone()).await;
        let total_article_price =
            currency_service::money::slow_multiply(&localized_price, item.quantity);

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
        expiration_years: (0..=4).map(|i| current_year + i).collect(),
    };

    let total_context = template_context.extend_with_handler_context(local_context);

    (
        ContentType::HTML,
        Template::render("cart.html.tera", total_context),
    )
}

#[derive(FromForm)]
pub struct AddToCartForm {
    quantity: i32,
    product_id: String,
}

#[route(POST, uri = "/", data = "<cart_form>")]
pub async fn add_to_cart(cookie_jar: &CookieJar<'_>, cart_form: Form<AddToCartForm>) -> Redirect {
    println!("GETTING TO ADD");
    //TODO: Validate the form
    let t_ctx = tarpc::context::current();
    let product = get_product(t_ctx, cart_form.product_id.clone()).await;
    let session_id = user_session_id(cookie_jar);
    add_item(t_ctx, session_id, product.id, cart_form.quantity).await;
    Redirect::found(uri!("/cart"))
}

#[route(POST, uri = "/empty")]
pub async fn empty_cart(cookie_jar: &CookieJar<'_>) -> Redirect {
    let t_ctx = tarpc::context::current();
    let session_id = user_session_id(cookie_jar);
    delete_cart(t_ctx, session_id).await;
    Redirect::found(uri!("/"))
}

#[derive(FromForm)]
pub struct CheckoutForm {
    email: String,
    street_address: String,
    zip_code: i64,
    city: String,
    state: String,
    country: String,
    credit_card_number: String,
    credit_card_expiration_month: i32,
    credit_card_expiration_year: i32,
    credit_card_cvv: i32,
    store_payment_info: bool
}

#[derive(serde::Serialize)]
struct OrderRenderingContext{
    show_currency: bool,
    currencies: Vec<String>,
    order: OrderResult,
    total_paid: Money,
    recommendations: Vec<Product>
}

#[route(POST, uri = "/checkout", data = "<checkout_form>")]
pub async fn checkout(
    checkout_form: Form<CheckoutForm>,
    cookie_jar: &CookieJar<'_>,
    template_context: SharedRenderingContext
) -> (ContentType, Template) {
    let user_currency = current_user_currency(cookie_jar);
    let session_id = user_session_id(cookie_jar);
    let t_ctx = tarpc::context::current();
    let cc = CreditCardInfo {
        credit_card_number: checkout_form.credit_card_number.clone(),
        credit_card_expiration_month: checkout_form.credit_card_expiration_month,
        credit_card_expiration_year: checkout_form.credit_card_expiration_year,
        credit_card_cvv: checkout_form.credit_card_cvv,
    };

    let address = Address {
        street_address: checkout_form.street_address.clone(),
        city: checkout_form.city.clone(),
        state: checkout_form.state.clone(),
        country: checkout_form.country.clone(),
        zip_code: checkout_form.zip_code as i32,
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

    let recommendations = list_recommendations(t_ctx, session_id.clone(), Vec::new()).await;

    let items = order.items.clone();
    let total_price = items
        .iter()
        .map(|item| currency_service::money::slow_multiply(&item.cost, item.item.quantity))
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

    (ContentType::HTML, Template::render("order.html.tera", total_context))



}
