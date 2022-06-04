#![allow(clippy::unused_io_amount)]

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    /// Create new Client
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// Read message/buffer from Client
    pub async fn read(&mut self) -> anyhow::Result<String> {
        let mut buf = [0; 1024];

        self.stream.read(&mut buf).await?;

        let encoded = String::from_utf8(buf.to_vec())?.replace('\0', "");

        Ok(encoded)
    }

    /// Send message to Client
    pub async fn send(&mut self, content: &str) -> anyhow::Result<()> {
        self.stream.write_all(format!("{content}\n\r").as_bytes()).await?;
        Ok(())
    }
}
