use std::{
    collections::HashMap,
    fmt,
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};

use tungstenite::{accept, Message, WebSocket};

use super::run::PLUGINS_MANAGER;
use crate::plugins::{manager::PluginsManagerType, prelude::EventType};

/// Max length of a TCP and UDP packet
pub const MAX_PACKET_LEN: usize = 65536;

/// Client struct
#[derive(Debug, Clone)]
pub struct Client {
    /// ID of the client
    pub id: usize,
    /// Connection stream of the client
    pub stream: ClientStream,
    /// Custom Client Map
    pub map: Arc<Mutex<HashMap<String, ClientMapValue>>>,
    /// Plugins Manager
    pub plugins_manager: PluginsManagerType,
}

// impl Drop for Client {
//     fn drop(&mut self) {
//         if let Err(err) = self.close() {
//             error!("Failed to close client connection: {err}");
//         }
//     }
// }

/// Value type of the client map entry
#[derive(Debug, Clone)]
pub enum ClientMapValue {
    String(String),
    Array(Vec<String>),
    Bool(bool),
    Int(isize),
    UInt(usize),
}

/// Connection stream of the client
#[derive(Debug, Clone)]
pub enum ClientStream {
    /// TCP stream
    TCP(Arc<TcpStream>),
    /// WebSocket stream
    WebSocket(Arc<Mutex<WebSocket<TcpStream>>>),
}

impl From<TcpStream> for Client {
    fn from(stream: TcpStream) -> Self {
        Self {
            id: 0,
            stream: ClientStream::TCP(Arc::new(stream)),
            map: Arc::new(Mutex::new(HashMap::new())),
            plugins_manager: PLUGINS_MANAGER.clone(),
        }
    }
}

impl From<WebSocket<TcpStream>> for Client {
    fn from(stream: WebSocket<TcpStream>) -> Self {
        Self {
            id: 0,
            stream: ClientStream::WebSocket(Arc::new(Mutex::new(stream))),
            map: Arc::new(Mutex::new(HashMap::new())),
            plugins_manager: PLUGINS_MANAGER.clone(),
        }
    }
}

impl Client {
    /// Create a new TCP Client instance
    pub fn new_tcp(stream: TcpStream, id: usize) -> Self {
        let mut client = Self::from(stream);

        client.id = id;

        client
    }

    /// Create a new WebSocket Client instance
    pub fn new_websocket(stream: TcpStream, id: usize) -> anyhow::Result<Self> {
        let websocket = accept(stream)?;

        let mut client = Self::from(websocket);

        client.id = id;

        Ok(client)
    }

    /// Recieve a message from the client
    pub fn read(&self) -> anyhow::Result<String> {
        // read the message from the stream
        let mut msg = match &self.stream {
            ClientStream::TCP(stream) => {
                // allocate an empty buffer
                let mut buf = [0; MAX_PACKET_LEN];

                // read the message and get length of it
                let len = stream.as_ref().read(&mut buf)?;

                // select only used bytes in the buffer
                let buf = &buf[0..len];

                // decode buffer (&[u8]) to a String
                String::from_utf8(buf.to_vec())?
            },
            ClientStream::WebSocket(stream) => {
                // read the message from the stream
                let msg = stream.lock().unwrap().read_message()?;

                // decode message to a String
                msg.to_string()
            },
        };

        // remove new line characters
        while msg.ends_with('\n') || msg.ends_with('\r') {
            msg.pop();
        }

        Ok(msg)
    }

    /// Send a message to the client
    pub fn send<S>(&self, msg: S) -> anyhow::Result<()>
    where
        S: ToString,
        S: fmt::Display,
    {
        // convert the message into a string
        let msg = msg.to_string();

        // convert the message into bytes to send it
        let buf = msg.as_bytes();

        // send the message
        match &self.stream {
            ClientStream::TCP(stream) => stream.as_ref().write_all(buf)?,
            ClientStream::WebSocket(stream) => {
                stream.lock().unwrap().write_message(Message::from(msg))?
            },
        }

        Ok(())
    }

    /// Returns the socket address of the remote peer of this connection.
    pub fn peer_addr(&self) -> anyhow::Result<SocketAddr> {
        let addr = match &self.stream {
            ClientStream::TCP(stream) => stream.peer_addr()?,
            ClientStream::WebSocket(stream) => stream.lock().unwrap().get_ref().peer_addr()?,
        };

        Ok(addr)
    }

    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    pub fn flush(&self) -> anyhow::Result<()> {
        match &self.stream {
            ClientStream::TCP(stream) => stream.as_ref().flush()?,
            ClientStream::WebSocket(_stream) => {},
        }

        Ok(())
    }

    /// Close the client connection
    pub fn close(&self) -> anyhow::Result<()> {
        match &self.stream {
            ClientStream::TCP(stream) => stream.shutdown(Shutdown::Both)?,
            ClientStream::WebSocket(stream) => stream.lock().unwrap().close(None)?,
        }

        Ok(())
    }

    /// Inserts a key-value pair into the map.
    pub fn insert_key<S>(&self, key: S, value: ClientMapValue) -> Option<ClientMapValue>
    where
        S: ToString,
    {
        self.map.lock().unwrap().insert(key.to_string(), value)
    }

    /// Returns the value from the key.
    pub fn get_value<S>(&self, key: S) -> Option<ClientMapValue>
    where
        S: ToString,
    {
        self.map.lock().unwrap().get(&key.to_string()).cloned()
    }

    /// Delete key from the map.
    pub fn delete_key<S>(&self, key: S) -> Option<ClientMapValue>
    where
        S: ToString,
    {
        self.map.lock().unwrap().remove(&key.to_string())
    }

    pub async fn run_events(&self, event_type: EventType) -> anyhow::Result<()> {
        for event in self.plugins_manager.events.iter() {
            if event.event() == event_type {
                event.execute(self).await?;
            }
        }

        Ok(())
    }
}
