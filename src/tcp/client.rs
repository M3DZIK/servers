#![allow(clippy::unused_io_amount)]

use std::{
    io::{self, Read, Write},
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
    pub fn read(&mut self) -> anyhow::Result<String> {
        // allocate an empty buffer of length 1024 bytes
        let mut buf = [0; 1024];

        // read buffer from stream
        self.stream.read(&mut buf)?;

        // encode &[u8] to a String and replace null spaces (empty `\0` bytes)
        let decoded = String::from_utf8(buf.to_vec())?.replace('\0', "");

        Ok(decoded)
    }

    /// Send message to Client
    pub fn send(&mut self, content: &str) -> io::Result<()> {
        // add a new line at the end of the content
        let content = format!("{content}\n\r");

        // send message
        self.stream.write_all(content.as_bytes())
    }
}
