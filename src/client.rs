use crate::configuration::Configuration;
use protocol::AsyncConnection;

#[derive(Clone)]
pub struct Client {
    configuration: Configuration,
}

impl Client {
    fn new(configuration: Configuration) -> Client {
        Self { configuration }
    }

    // establish connection using given stream and return it as session
    async fn connect<S: protocol::Stream>(&self, stream: S) {
        let mut connection = AsyncConnection::from(stream);

        // write client id
        connection
            .write_client_id(self.configuration.client_id.clone())
            .await
            .unwrap();

        // read server id
        let server_id = connection.read_server_id().await.unwrap();

        // create channels for handle and session
        let (handle_sender, session_receiver) = tokio::sync::mpsc::channel(10);
        let (session_sender, handle_receiver) = tokio::sync::mpsc::unbounded_channel();

    }
}
