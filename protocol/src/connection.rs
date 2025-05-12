use crate::error::Error;
use log::{debug, info};
use tokio::io::{
    AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt,
};

pub trait Stream: AsyncRead + AsyncWrite + Unpin + Send {}
pub trait BufferedStream: Stream + AsyncBufRead {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Stream for T {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send + AsyncBufRead> BufferedStream for T {}

pub struct AsyncConnection<S: BufferedStream> {
    pub stream: S,
}

impl<S: BufferedStream> From<S> for AsyncConnection<S> {
    fn from(value: S) -> Self {
        Self { stream: value }
    }
}

impl<S: BufferedStream> AsyncConnection<S> {
    /// flush the buffered stream
    pub async fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush().await
    }

    /// write client id to stream
    pub async fn write_client_id(&mut self, id: String) -> std::io::Result<()> {
        self.stream.write_all(format!("{id}\r\n").as_bytes()).await
    }

    /// read incoming server id from stream and return it as string
    pub async fn read_server_id(&mut self) -> Result<String, Error> {
        let mut buffer = String::new();

        loop {
            let received_bytes = self.stream.read_line(&mut buffer).await?;

            // reached end of file
            if received_bytes == 0 {
                return Err(Error::Disconnect);
            }

            let Some(buffer_stripped) = buffer.strip_suffix("\n").map(|v| v.to_string()) else {
                continue;
            };

            if buffer_stripped.starts_with("SSH-2.0-") {
                let buffer_stripped = buffer.strip_suffix("\n").unwrap().to_string();
                return if let Some((id, _)) = buffer_stripped.split_once(" ") {
                    // split line into id and comment
                    Ok(id.to_string())
                } else {
                    // no comment received
                    Ok(buffer_stripped)
                };
            }

            // read next line
            buffer.clear();
        }
    }
}
