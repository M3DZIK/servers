use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "servers",
    version = env!("CARGO_PKG_VERSION"),
    about = "A simple TCP server for client which can be extended with plugins."
)]
pub struct Cli {
    #[clap(
        short = 'h',
        long = "host",
        default_value = "0.0.0.0",
        help = "Tcp server host",
        display_order = 1
    )]
    pub host: String,

    #[clap(
        short = 'p',
        long = "port",
        default_value = "9999",
        help = "Tcp server port [set 0 to random]",
        display_order = 2
    )]
    pub port: String,

    #[clap(
        short = 'w',
        long = "ws-port",
        default_value = "9998",
        help = "WebSocket server port [set 0 to random]",
        display_order = 3
    )]
    pub ws_port: String,

    #[clap(
        long = "disable-websocket",
        help = "Disable WebSocket proxy to Tcp",
        display_order = 4
    )]
    pub ws_disable: bool,
}
