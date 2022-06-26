#![allow(clippy::unused_io_amount)]

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

use super::MAX_PACKET_LEN;

/// Handle WebSocket connection
pub async fn handle_websocket(stream: TcpStream, tcp_port: String) -> anyhow::Result<()> {
    info!("New WebSocket Client: {}", stream.peer_addr()?);

    // accept connection as WebSocket
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;

    // connect to Tcp server
    let tcp_stream = TcpStream::connect(format!("0.0.0.0:{}", tcp_port)).await?;

    // split streams
    let (tcp_read, tcp_write) = tcp_stream.into_split();
    let (ws_write, ws_read) = ws_stream.split();

    // tcp read -> ws write
    tokio::spawn(tcp_to_ws(tcp_read, ws_write));

    // ws read -> tcp write
    ws_to_tcp(tcp_write, ws_read).await?;

    Ok(())
}

/// Tcp read -> WebSocket write
async fn tcp_to_ws(
    mut tcp_read: OwnedReadHalf,
    mut ws_write: SplitSink<WebSocketStream<TcpStream>, Message>,
) -> anyhow::Result<()> {
    // allocate an empty buffer
    let mut buf = [0; MAX_PACKET_LEN];

    loop {
        // read buffer from tcp
        let len = tcp_read.read(&mut buf).await?;

        if len > 0 {
            // select only used bytes from the buffer
            let recv_buf = &buf[0..len];
            // covert &[u8] buffer to a vector
            let recv_vec = recv_buf.to_vec();
            // create a `Message` type from buffer Vec<u8>
            let msg = Message::Binary(recv_vec);

            // write buffer to websocket
            ws_write.send(msg).await?;
        }
    }
}

/// WebSocket read -> Tcp write
async fn ws_to_tcp(
    mut tcp_write: OwnedWriteHalf,
    mut ws_read: SplitStream<WebSocketStream<TcpStream>>,
) -> anyhow::Result<()> {
    while let Some(msg) = ws_read.next().await {
        // handle error in the message
        let msg = msg?;
        // create a buffer from a message
        let buf = msg.into_data();

        // write buffer to tcp
        tcp_write.write(&buf).await?;
    }

    Ok(())
}
