use cart_service::types::CartItem;
use checkout_service::types::Address;
use email_service::types::Money;
use shipping_service::service::TahiniShippingServiceClient;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use alohomora::bbox::BBox;
use alohomora::policy::NoPolicy;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;
use tahini_tarpc::transport::new_tahini_client_transport as new_transport;

static SHIPPING_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50058);
static SHIPPING_CLIENT: OnceLock<TahiniShippingServiceClient> = OnceLock::new();

pub(super) async fn initialize_shipping_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&SHIPPING_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = TahiniShippingServiceClient::new(Default::default(), transport).spawn().await;
    if let Err(_) = SHIPPING_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn ship_order(
    ctx: tarpc::context::Context,
    address: Address,
    items: Vec<CartItem>,
) -> BBox<String, NoPolicy> {
    match SHIPPING_CLIENT.get() {
        None => unreachable!("Shipping Client should have been initialized before calling its API"),
        Some(shipping_client) => {
            shipping_client
                .ship_order(
                    ctx,
                    shipping_service::types::ShipOrderRequest {
                        address,
                        items: items,
                    },
                )
                .await
                .expect("Couldn't connect to shipping client")
                .tracking_id
        }
    }
}

pub async fn get_quote(
    ctx: tarpc::context::Context,
    address: Address,
    items: Vec<CartItem>,
) -> Money {
    match SHIPPING_CLIENT.get() {
        None => unreachable!("Shipping Client should have been initialized before calling its API"),
        Some(shipping_client) => {
            shipping_client
                .get_quote(
                    ctx,
                    shipping_service::types::GetQuoteRequest { address, items },
                )
                .await
                .expect("Couldn't connect to shipping client")
                .cost_usd
        }
    }
}
