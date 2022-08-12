use clap::Parser;
use servers::tcp::server;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Cli {
    #[clap(
        short = 'i',
        long = "host",
        help = "Server host",
        default_value = "0.0.0.0",
        display_order = 1
    )]
    host: String,
    #[clap(
        short = 't',
        long = "tcp-port",
        help = "TCP server port",
        default_value = "9999",
        display_order = 2
    )]
    tcp_port: u16,
    #[clap(
        short = 'w',
        long = "websocket-port",
        help = "WebSocket server port",
        default_value = "9998",
        display_order = 3
    )]
    ws_port: u16,
}

fn main() {
    tracing_subscriber::fmt().init();

    let args = Cli::parse();

    let tcp_host = format!("{host}:{port}", host = args.host, port = args.tcp_port);
    let ws_host = format!("{host}:{port}", host = args.host, port = args.ws_port);

    server::run(tcp_host, ws_host).expect("failed to start tcp server");
}
