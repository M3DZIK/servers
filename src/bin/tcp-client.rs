use std::{
    io::{self, Read, Write},
    net::TcpStream,
    sync::Arc,
    thread,
};

use clap::Parser;
use servers::server::MAX_PACKET_LEN;

#[derive(Debug, Parser)]
#[clap(
    name = "tcp-client",
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Cli {
    #[clap(
        short = 'i',
        long = "--host",
        help = "Server host",
        default_value = "0.0.0.0",
        display_order = 1
    )]
    host: String,

    #[clap(
        short = 'p',
        long = "--port",
        help = "Server port",
        default_value = "9999",
        display_order = 2
    )]
    port: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let addr = format!("{}:{}", args.host, args.port);

    println!("Connecting to {}...", addr);

    let stream = TcpStream::connect(addr)?;

    println!("Connected!");

    let stream = Arc::new(stream);

    let reader = stream.clone();
    let writer = stream;

    // read the output from the server to write to the stdout
    thread::spawn(move || {
        let mut buf = [0; MAX_PACKET_LEN];

        // read buffer from the server
        while let Ok(buf_len) = reader.as_ref().read(&mut buf) {
            // ignore unused bytes
            let buf = &buf[0..buf_len];

            // decode buffer from &[u8] to a String
            let mut buf_str = String::from_utf8(buf.to_vec()).unwrap();

            // delete new line characters from the buffer
            buf_str = buf_str.replace('\n', "");
            buf_str = buf_str.replace('\r', "");

            println!("{}", buf_str);
        }
    });

    // create a new stdin handler
    let stdin = io::stdin();

    // send command from stdin
    loop {
        let mut buf = String::new();

        // read buffer from stdin
        stdin.read_line(&mut buf)?;

        // remove new line characters
        while buf.ends_with('\n') || buf.ends_with('\r') {
            buf.pop();
        }

        // send the buffer to the server
        writer.as_ref().write_all(buf.as_bytes())?;
    }
}
