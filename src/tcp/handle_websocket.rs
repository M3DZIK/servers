#![allow(clippy::unused_io_amount)]

use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
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
    let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();
    let (mut ws_write, mut ws_read) = ws_stream.split();

    // tcp read -> ws write
    tokio::spawn(async move {
        // allocate an empty buffer
        let mut buf = [0; MAX_PACKET_LEN];

        loop {
            // read buffer from tcp
            let len = tcp_read.read(&mut buf).await.unwrap();

            if len > 0 {
                // select only used bytes from the buffer
                let recv_buf = &buf[0..len];
                // covert &[u8] buffer to a vector
                let recv_vec = recv_buf.to_vec();
                // create a `Message` type from buffer Vec<u8>
                let msg = Message::Binary(recv_vec);

                // write buffer to websocket
                ws_write.send(msg).await.unwrap();
            }
        }
    });

    // ws read -> tcp write
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
