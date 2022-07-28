use clap::Parser;
use servers::{
    plugins::loader,
    tcp::{handle_connection, handle_websocket, Client},
};
use tokio::net::TcpListener;
use tracing::{info, error};

#[derive(Parser)]
#[clap(
    name = "servers",
    about = "A simple TCP server for client which can be extended with plugins."
)]
struct Cli {
    #[clap(
        short = 'h',
        long = "host",
        default_value = "0.0.0.0",
        help = "Tcp server host",
        display_order = 1
    )]
    host: String,
    #[clap(
        short = 'p',
        long = "port",
        default_value = "9999",
        help = "Tcp server port [set 0 to random]",
        display_order = 2
    )]
    port: String,

    #[clap(
        short = 'w',
        long = "ws-port",
        default_value = "9998",
        help = "WebSocket server port [set 0 to random]",
        display_order = 3
    )]
    ws_port: String,

    #[clap(
        long = "disable-websocket",
        help = "Disable WebSocket proxy to Tcp",
        display_order = 4
    )]
    ws_disable: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init better panic
    better_panic::install();
    // init logger
    tracing_subscriber::fmt().init();

    // parse cli args
    let cli = Cli::parse();

    // if enabled start WebSocket server
    if !cli.ws_disable {
        tokio::spawn(start_ws_server(
            cli.host.clone(),
            cli.ws_port,
            cli.port.clone(),
        ));
    }

    // start tcp server
    start_tcp_server(cli.host, cli.port).await?;

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
            let ip = client.stream.peer_addr().unwrap();

            match handle_connection(client, plugin_manager).await {
                Ok(_) => (),
                Err(err) => error!("Client {}, {}", ip, err),
            }
        });
    }

    // server for a unexpectedly reason be terminated
    panic!("Server unexpectedly terminated!")
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
            let ip = stream.peer_addr().unwrap();

            match handle_websocket(stream, tcp_port).await {
                Ok(_) => (),
                Err(err) => error!("Client {}, {}", ip, err),
            }
        });
    }

    // server for a unexpectedly reason be terminated
    panic!("Server unexpectedly terminated!")
}
