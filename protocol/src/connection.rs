use crate::error::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub trait Stream: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> Stream for T {}

pub struct AsyncConnection<S: Stream> {
    stream: S,
}

impl<S: Stream> From<S> for AsyncConnection<S> {
    fn from(value: S) -> Self {
        Self { stream: value }
    }
}

impl<S: Stream> AsyncConnection<S> {
    /// write client id to stream
    pub async fn write_client_id(&mut self, id: String) -> std::io::Result<()> {
        self.stream.write_all(id.as_bytes()).await
    }

    /// read incoming server id from stream and return it as string
    pub async fn read_server_id(&mut self) -> Result<String, Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut total: usize = 0;
        let mut bytes_read: usize = 0;
        let mut i: usize = 0;

        loop {
            let n = self.stream.read(&mut buffer[total..]).await?;
            if n == 0 {
                return Err(Error::Disconnect);
            }

            total += n;

            // update bytes_read
            loop {
                if i >= total - 1 {
                    break;
                }

                if buffer[i + 1] == b'\n' {
                    bytes_read = i + 2;
                    if buffer[i] == b'\r' {
                        // '\r\n'
                        break;
                    }

                    // '\n'
                    i += 1;
                } else {
                    i += 1;
                }
            }

            if bytes_read > 0 {
                // received full line
                if i >= 8 && buffer.get(0..8) == Some(b"SSH-2.0-") {
                    return Ok(String::from_utf8(buffer.clone())?);
                }

                // https://tools.ietf.org/html/rfc4253#section-4.2
                // reset counters and read the next line
                total = 0;
                bytes_read = 0;
            }
        }
    }
}
