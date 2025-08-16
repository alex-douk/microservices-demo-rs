use email_service::service::EmailServiceClient;
use email_service::types::OrderResult;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::OnceLock;
use tarpc::tokio_serde::formats::Json;
use tarpc::tokio_util::codec::LengthDelimitedCodec;
use tokio::net::TcpStream;

use tarpc::serde_transport::new as new_transport;


static EMAIL_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 50061);
static EMAIL_CLIENT: OnceLock<EmailServiceClient > = OnceLock::new();


pub(super) async fn initialize_email_client() {
    let codec_builder = LengthDelimitedCodec::builder();
    let stream = TcpStream::connect(&EMAIL_ADDRESS).await.unwrap();
    let transport = new_transport(codec_builder.new_framed(stream), Json::default());
    let client = EmailServiceClient::new(Default::default(), transport).spawn();
    if let Err(_) = EMAIL_CLIENT.set(client) {
        panic!("Client connection already exists");
    }
}

pub async fn send_order_confirmation(ctx: tarpc::context::Context, email: String, order_result: OrderResult) {
    match EMAIL_CLIENT.get() {
        None => unreachable!("Email Client should have been initialized before calling its API"),
        Some(email_client) => email_client
            .send_order_confirmation(
                ctx,
                email_service::types::SendOrderConfirmationRequest { email , order: order_result }
            )
            .await
            .expect("Couldn't connect to email client"),
    }


}
