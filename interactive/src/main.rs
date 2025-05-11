use log::{LevelFilter, info, log};
use rushell::client::Client;
use rushell::configuration::Configuration;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    colog::default_builder()
        .filter_level(LevelFilter::Trace)
        .init();

    info!("starting interactive");

    let client = Client::new(Configuration::builder().build());
    let stream = TcpStream::connect("0.0.0.0:22").await.unwrap();

    client.connect(stream).await;
}
