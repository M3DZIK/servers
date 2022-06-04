use std::net::TcpListener;

use crate::{
    loader,
    tcp::{handle_connection, Client},
};

pub fn start_server(host: &str, port: &str) -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind(format!("{host}:{port}"))?;

    println!("Tcp server started at: {}", listener.local_addr()?);

    // load plugins and commands
    let (_plugin_manager, commands_manager) = loader()?;

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept() {
        let client = Client::new(stream);

        let commands_manager = commands_manager.clone();

        // handle client connection in new thread
        tokio::spawn(handle_connection(client, commands_manager));
    }

    Ok(())
}
