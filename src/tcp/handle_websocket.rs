#![allow(clippy::unused_io_amount)]

use futures_util::{SinkExt, StreamExt};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tungstenite::Message;

use super::MAX_PACKET_LEN;

/// Handle WebSocket connection
pub async fn handle_websocket(stream: TcpStream, tcp_port: String) -> anyhow::Result<()> {
    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
    let tcp_stream = TcpStream::connect(format!("0.0.0.0:{}", tcp_port)).await?;

    let (mut tcp_read, mut tcp_write) = tcp_stream.into_split();
    let (mut ws_write, mut ws_read) = ws_stream.split();

    tokio::spawn(async move {
        let mut buf = [0; MAX_PACKET_LEN];

        loop {
            let len = tcp_read.read(&mut buf).await.unwrap();

            if len > 0 {
                let recv_buffer = &buf[0..len];
                let recv_vec: Vec<u8> = recv_buffer.to_vec();
                let msg = Message::Binary(recv_vec);
                ws_write.send(msg).await.unwrap();
            }
        }
    });

    while let Some(msg) = ws_read.next().await {
        let msg = msg?;
        let buffer: &[u8] = &msg.into_data();
        tcp_write.write(buffer).await?;
    }

    Ok(())
}
