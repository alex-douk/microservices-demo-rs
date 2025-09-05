use futures::StreamExt;
use std::net::{IpAddr, Ipv4Addr};
use std::{fs::File, io::Read};
use alohomora::policy::AnyPolicyDyn;
use alohomora::pure::{execute_pure, PrivacyPureRegion};
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpListener;
use hoodini_server::CLIENT_MAP;
use tahini_tarpc::server::{TahiniChannel, TahiniBaseChannel};

use futures::Future;
use tahini_tarpc::transport::new_tahini_server_transport as new_transport;

use email_service::{service::EmailService};
use minijinja::{context, Environment};
use email_service::types::{OrderResultOut, UsableSendOrderConfirmationRequest};

static SERVER_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50061);

#[derive(Clone)]
pub struct EmailServer;

impl EmailService for EmailServer {
    async fn send_order_confirmation(
        self,
        _context: tarpc::context::Context,
        confirmation_request: UsableSendOrderConfirmationRequest,
    ) {
        let mut env = Environment::new();
        let mut template_str = String::new();
        let _ = File::open("templates/confirmation.html")
            .expect("Couldn't find template file")
            .read_to_string(&mut template_str);
        let _ = env
            .add_template("confirmation.html", &template_str)
            .unwrap();
        let res = env.get_template("confirmation.html").unwrap();
        let order = confirmation_request.order;
        let _ = execute_pure::<dyn AnyPolicyDyn, _, _, _>(
            order,
            PrivacyPureRegion::new(|order: OrderResultOut| {
                let context = context!(order => order);
                res.render(context)
            })
        );

        //TODO: Perhaps have a print of the rendered template to showcase the "wanted" privacy
        //leakage of this service.
        /*
        This does not compile anymore
        println!(
            "A request to send order confirmation email to {} has been received.",
            confirmation_request.email
        );
         */
    }
}

pub(crate) async fn wait_upon(fut: impl Future<Output = ()> + Send + 'static) {
    fut.await
}

#[tokio::main]
async fn main() {
    let server = EmailServer;
    let listener = TcpListener::bind(&SERVER_ADDRESS).await.unwrap();
    let codec_builder = LengthDelimitedCodec::builder();

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
