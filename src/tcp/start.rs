use std::net::TcpListener;

use crate::{
    plugins::loader,
    tcp::{handle_connection, Client},
};

#[tokio::main]
pub async fn start_server(host: &str, port: &str) -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind(format!("{host}:{port}"))?;

    println!("Tcp server started at: {}", listener.local_addr()?);

    // load plugins and commands
    let (command_manager, _plugin_manager, event_manager) = loader()?;

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept() {
        let client = Client::new(stream);

        // clone `CommandManager`
        let command_manager = command_manager.clone();
        // clone `EventManager`
        let event_manager = event_manager.clone();

        // handle client connection in new thread
        tokio::spawn(handle_connection(client, command_manager, event_manager));
    }

    Ok(())
}
