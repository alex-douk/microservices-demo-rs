use cart_service::service::CartService;

use cart_service::types::{Cart, CartItem};
use futures::StreamExt;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, RwLock};
use alohomora::bbox::BBox;
use alohomora::pcr::{execute_pcr, PrivacyCriticalRegion, Signature};
use alohomora::policy::{AnyPolicy, AnyPolicyDyn, NoPolicy};
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use tahini_tarpc::server::{TahiniChannel, TahiniBaseChannel};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;

use futures::Future;
use hoodini_server::CLIENT_MAP;
use tahini_tarpc::transport::new_tahini_server_transport as new_transport;

#[derive(Clone)]
struct CartServer {
    //You'd think there's a TTL on the cache, but apparently not(!!!)
    // TODO(babman): the key is the user id, which is BBoxed.
    //               meaning that writing (not reading) to the map is a PCR.
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
        let cart = get_cart_req.user_id.clone().into_ppr(PrivacyPureRegion::new(|user_id: String| {
            match cache_read.get(&user_id) {
                None => Cart::new(get_cart_req.user_id),
                Some(cart) => cart.clone(),
            }
        }));
        // TODO(babman): this might give us a headachen when policies are real.
        //               we might need an API to flatten BBoxs by doing conjunction on their policies.
        cart.discard_box()
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

        empty_cart_req.user_id.into_pcr(
            PrivacyCriticalRegion::new(
                |user_id: String, _: NoPolicy, _c: ()| {
                cache_write.remove(&user_id)
                },
                Signature {
                    username: "",
                    signature: ""
                }
            ),
            ()
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

        let (user_id, item) = (add_item_req.user_id, add_item_req.item);
        user_id.into_pcr(
            PrivacyCriticalRegion::new(
                |user_id: String, p: NoPolicy, _c: ()| {
                    match write_lock.get_mut(&user_id) {
                        //If no cart for the current user, create a new cart with only the requested item
                        None => {
                            let mut new_cart = Cart::new(BBox::new(user_id.clone(), p));
                            new_cart.items.push(item);
                            write_lock.insert(user_id, new_cart);
                        }
                        Some(cart) => {
                            // If cart already exists, check if item is in cart and increase its count. If not,
                            // create new entry
                            let old_item = cart
                                .items
                                .iter_mut()
                                .find(|curent_item| {
                                    helper(curent_item, &item)
                                });

                            match old_item {
                                None => cart.items.push(item),
                                Some(old_item) => {
                                    old_item.quantity = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
                                        (old_item.quantity.clone(), item.quantity),
                                        PrivacyPureRegion::new(|(q1, q2): (i64, i64)| {
                                            q1 + q2
                                        })
                                    ).unwrap().specialize_policy().unwrap();
                                }
                            }
                        }
                    }
                },
                Signature {
                    username: "",
                    signature: ""
                }
            ),
            ()
        );
    }
}

fn helper(item1: &CartItem, item2: &CartItem) -> bool {
    execute_pcr::<dyn AnyPolicyDyn, _, _, _, _>(
        (item1.product_id.clone(), item2.product_id.clone()),
        PrivacyCriticalRegion::new(
            |(pid1, pid2): (String, String), _: AnyPolicy, _: ()| {
                pid1 == pid2
            },
            Signature {
                username: "",
                signature: ""
            }
        ),
        ()
    ).unwrap()
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
        let transport = new_transport(framed, Json::default(), (*CLIENT_MAP).clone());
        let fut = TahiniBaseChannel::with_defaults(transport)
            .execute(server.clone().serve())
            .for_each(wait_upon);
        tokio::spawn(fut);
    }
}
