use cart_service::service::TahiniCartServiceClient;
use cart_service::types::Cart;
use std::net::{IpAddr, Ipv4Addr};
use std::sync:: OnceLock;
use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;
use tahini_tarpc::transport::new_tahini_client_transport as new_transport;

static CART_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50054);
static CART_CLIENT: OnceLock<TahiniCartServiceClient> = OnceLock::new();

pub(super) async fn initialize_cart_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&CART_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = TahiniCartServiceClient::new(Default::default(), transport).spawn().await;
    if let Err(_) = CART_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn get_cart(ctx: tarpc::context::Context, user_id: BBox<String, NoPolicy>) -> Cart {
    match CART_CLIENT.get() {
        Some(cart_client) => cart_client
            .get_cart(ctx, cart_service::types::GetCartRequest { user_id })
            .await
            .expect("Couldn't connect to cart client"),
        None => unreachable!("Cart Client should have been initialized before calling its API"),
    }
}

pub async fn delete_cart(ctx: tarpc::context::Context, user_id: BBox<String, NoPolicy>) {
    match CART_CLIENT.get() {
        Some(cart_client) => cart_client
            .empty_cart(ctx, cart_service::types::EmptyCartRequest { user_id })
            .await
            .expect("Couldn't connect to cart client"),
        None => unreachable!("Cart Client should have been initialized before calling its API"),
    }
}

// pub async fn add_item(ctx: tarpc::context::Context, user_id: String, item: CartItem) {
//     match CART_CLIENT.get() {
//         Some(cart_client) => cart_client
//             .add_item(ctx, cart_service::types::AddItemRequest { user_id, item })
//             .await
//             .expect("Couldn't connect to cart client"),
//         None => unreachable!("Cart Client should have been initialized before calling its API"),
//     }
// }
