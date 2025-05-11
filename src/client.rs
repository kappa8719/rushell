use crate::configuration::Configuration;
use log::{debug, info};
use protocol::AsyncConnection;
use tokio::io::{BufReader, BufStream};

#[derive(Clone)]
pub struct Client {
    configuration: Configuration,
}

impl Client {
    pub fn new(configuration: Configuration) -> Client {
        Self { configuration }
    }

    // establish connection using given stream and return it as session
    pub async fn connect<S: protocol::Stream>(&self, stream: S) {
        let stream = BufStream::new(stream);
        let mut connection = AsyncConnection::from(stream);

        // initiate version exchange

        // write client id
        connection
            .write_client_id(self.configuration.client_id.clone())
            .await
            .unwrap();

        connection.flush().await.unwrap();

        // read server id
        let server_id = connection.read_server_id().await.unwrap();

        info!("received server id: {}", server_id.clone());

        // create channels for handle and session
        // let (handle_sender, session_receiver) = tokio::sync::mpsc::channel(10);
        // let (session_sender, handle_receiver) = tokio::sync::mpsc::unbounded_channel();
    }
}
