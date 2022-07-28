use tokio::{net::TcpStream, io::{self, AsyncWriteExt, AsyncReadExt}};

/// Max size of a TCP packet
pub const MAX_PACKET_LEN: usize = 65536;

/// TCP Client
pub struct Client {
    /// TCP stream of this client
    pub stream: TcpStream,
}

impl Client {
    /// Create new Client
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Read message/buffer from the client
    pub async fn read(&mut self) -> anyhow::Result<String> {
        // allocate an empty buffer
        let mut buf = [0; MAX_PACKET_LEN];

        // read buffer from stream
        let len = self.stream.read(&mut buf).await?;

        // select only used bytes from the buffer
        let recv_buf = &buf[0..len];

        // encode buffer (&[u8]) to a String
        let decoded = String::from_utf8(recv_buf.to_vec())?;

        Ok(decoded)
    }

    /// Send message to the client
    pub async fn send(&mut self, content: &str) -> io::Result<()> {
        // add a new line at the end of the content
        let content = format!("{content}\n\r");

        // send message
        self.stream.write_all(content.as_bytes()).await
    }
}
