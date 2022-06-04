use clap::Parser;
use servers::tcp;

#[derive(Parser)]
#[clap(
    name = "servers",
    about = "Simple Tcp server that supports expansion via plugins"
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
    let cli = Cli::parse();

    // start tcp server
    tcp::start_server(&cli.host, &cli.port)?;

    Ok(())
}
