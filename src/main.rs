use clap::Parser;
use cli::Cli;
use servers::{
    logger,
    plugins::loader,
    tcp::{handle_connection, handle_websocket, Client},
};
use tokio::net::TcpListener;
use tracing::{error, info};

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    // parse cli args
    let args = Cli::parse();

    // if enabled start WebSocket server
    if !args.ws_disable {
        tokio::spawn(start_ws_server(
            args.host.clone(),
            args.ws_port,
            args.port.clone(),
        ));
    }

    // start tcp server
    start_tcp_server(args.host, args.port).await?;

    Ok(())
}

/// Start tcp server
async fn start_tcp_server(host: String, port: String) -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    info!("Tcp server started at: {}", listener.local_addr()?);

    // load plugins, commands and events
    let plugin_manager = loader()?;

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept().await {
        let client = Client::new(stream);
        let plugin_manager = plugin_manager.clone();

        // handle client connection in new thread
        tokio::spawn(async move {
            // get ip address of the client
            let ip = client
                .stream
                .peer_addr()
                .expect("failed to get peer address");

            if let Err(e) = handle_connection(client, plugin_manager).await {
                error!("Client {ip}: {e}")
            }
        });
    }

    // server for a unexpectedly reason be terminated
    panic!("TCP server unexpectedly terminated!")
}

/// Start WebSocket server
async fn start_ws_server(host: String, port: String, tcp_port: String) -> anyhow::Result<()> {
    // listen Tcp server
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;

    info!("WebSocket server started at: {}", listener.local_addr()?);

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept().await {
        let tcp_port = tcp_port.clone();
        tokio::spawn(async {
            // get ip address of the client
            let ip = stream.peer_addr().expect("failed to get peer address");

            if let Err(e) = handle_websocket(stream, tcp_port).await {
                error!("Client {ip}: {e}")
            }
        });
    }

    // server for a unexpectedly reason be terminated
    panic!("WebSocket server unexpectedly terminated!")
}
