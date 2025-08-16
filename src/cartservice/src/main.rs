use cart_service::service::CartService;

use cart_service::types::Cart;
use futures::StreamExt;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use futures::Future;
use tarpc::serde_transport::new as new_transport;

#[derive(Clone)]
struct CartServer {
    //You'd think there's a TTL on the cache, but apparently not(!!!)
    cache: Arc<RwLock<HashMap<String, Cart>>>,
}

impl CartServer {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50054);

impl CartService for CartServer {
    async fn get_cart(
        self,
        _context: tarpc::context::Context,
        get_cart_req: cart_service::types::GetCartRequest,
    ) -> Cart {
        let cache_read = self
            .cache
            .read()
            .expect("Couldn't acquire read lock. Cache is poisoned");
        match cache_read.get(&get_cart_req.user_id) {
            None => Cart::new(get_cart_req.user_id),
            Some(cart) => cart.clone(),
        }
    }

    async fn empty_cart(
        self,
        _context: tarpc::context::Context,
        empty_cart_req: cart_service::types::EmptyCartRequest,
    ) {
        let mut cache_write = self
            .cache
            .write()
            .expect("Couldn't acquire write lock. Cache is poisoned");
        cache_write.insert(
            empty_cart_req.user_id.clone(),
            Cart::new(empty_cart_req.user_id),
        );
    }

    async fn add_item(
        self,
        _context: tarpc::context::Context,
        add_item_req: cart_service::types::AddItemRequest,
    ) -> () {
        let mut write_lock = self
            .cache
            .write()
            .expect("Couldn't acquire write lock. Cache is poisoned");
        match write_lock.get_mut(&add_item_req.user_id) {
            //If no cart for the current user, create a new cart with only the requested item
            None => {
                let mut new_cart = Cart::new(add_item_req.user_id.clone());
                new_cart.items.push(add_item_req.item);
                write_lock.insert(add_item_req.user_id, new_cart);
            }
            Some(cart) => {
                //If cart already exists, check if item is in cart and increase its count. If not,
                //create new entry
                match cart
                    .items
                    .iter_mut()
                    .find(|item| item.product_id == add_item_req.item.product_id)
                {
                    None => cart.items.push(add_item_req.item),
                    Some(item) => item.quantity += add_item_req.item.quantity,
                }
            }
        }
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();
    let server = CartServer::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let framed = codec_builder.new_framed(stream);
        let transport = new_transport(framed, Json::default());
        let fut = BaseChannel::with_defaults(transport)
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
