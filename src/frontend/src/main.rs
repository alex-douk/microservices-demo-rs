use rocket::fs::relative;
use rocket::{fs::FileServer, routes, Build, Rocket};

use crate::handlers::{currency, home, product, cart, template_fairing};
use crate::rpcs::init_services;

mod handlers;
mod middleware;
mod rpcs;
pub mod utils;

fn prepare_server() -> Rocket<Build> {
    Rocket::build()
        .mount("/", routes![home::home, product::product_detail, home::logout])
        .mount("/setCurrency", routes![currency::set_currency])
        .mount("/cart", routes![cart::view_cart, cart::add_to_cart, cart::empty_cart, cart::checkout])
        .attach(middleware::EnsureSessionId)
        .attach(template_fairing())
        .mount("/static", FileServer::from(relative!("static")))
}

#[rocket::main]
async fn main() {
    init_services().await;
    if let Err(e) = prepare_server().launch().await {
        println!("Failed to launch the webserver!");
        drop(e)
    }
}
