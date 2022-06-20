use std::net::TcpListener;

use clap::Parser;
use servers::{
    plugins::loader,
    tcp::{handle_connection, Client},
};
use simple_logger::SimpleLogger;

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
}

fn main() -> anyhow::Result<()> {
    // init logger
    SimpleLogger::new().init()?;

    // parse cli args
    let cli = Cli::parse();

    // start tcp server
    start_server(&cli.host, &cli.port)?;

    Ok(())
}

/// Start tcp server
#[tokio::main]
async fn start_server(host: &str, port: &str) -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind(format!("{host}:{port}"))?;

    println!("Tcp server started at: {}", listener.local_addr()?);

    // load plugins, commands and events
    let plugin_manager = loader()?;

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept() {
        let client = Client::new(stream);

        // handle client connection in new thread
        tokio::spawn(handle_connection(client, plugin_manager.clone()));
    }

    // server for a unexpectedly reason be terminated
    panic!("Server unexpectedly terminated!")
}
