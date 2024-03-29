use std::{net::TcpListener, thread};

use anyhow::anyhow;
use async_std::task;
use futures::join;
use lazy_static::lazy_static;
use tracing::{error, info, span, Level};

use crate::{
    plugins::{
        self,
        prelude::{EventData, EventType},
        PluginsManagerType,
    },
    server::Client,
    CLIENTS, CLIENT_NEXT,
};

/// Plugins directory.
pub const PLUGINS_DIR: &str = "plugins";

lazy_static! {
    /// Plugin manager, where you can find loaded plugins, commands and events
    pub static ref PLUGINS_MANAGER: PluginsManagerType =
        plugins::loader(PLUGINS_DIR).expect("failed to load plugins");
}

/// Start servers
pub fn run(tcp_host: String, ws_host: String) -> anyhow::Result<()> {
    info!("Loaded {} plugins", PLUGINS_MANAGER.plugins.len());
    info!("Loaded {} commands", PLUGINS_MANAGER.commands.len());
    info!("Loaded {} events", PLUGINS_MANAGER.events.len());

    let tcp_child = task::spawn(async move {
        start_tcp(tcp_host).await.unwrap();
    });

    let ws_child = task::spawn(async move {
        start_websocket(ws_host).await.unwrap();
    });

    task::block_on(async {
        join!(tcp_child, ws_child);
    });

    Ok(())
}

/// Process client connection
async fn process(client: Client) -> anyhow::Result<()> {
    let client_addr = client.peer_addr()?;

    info!("Processing client connection: {}", client_addr);

    // run `onConnect` events
    client
        .run_events(EventType::OnConnect, EventData::None)
        .await?;

    loop {
        let buf = client.read()?;

        // functions for error handling see `if` below function
        async fn handle(client: &Client, buf: String) -> anyhow::Result<()> {
            // run `onSend` events
            client
                .run_events(EventType::OnSend, EventData::None)
                .await?;

            let mut args: Vec<&str> = buf.split_ascii_whitespace().collect();

            // if client sent an empty buffer
            if args.is_empty() {
                client.send("empty buffer")?;
                return Ok(());
            }

            let cmd = args[0];

            // remove command name from args
            args = args[1..args.len()].to_vec();

            // find command
            let command = client
                .plugins_manager
                .commands
                .iter()
                .enumerate()
                .find(|&(_i, command)| command.name() == cmd || command.aliases().contains(&cmd));

            // execute command, if command isn't blocked
            // to block a command return error in the `onCommand` event
            if let Some((_i, cmd)) = command {
                // run `onCommand` events
                if client
                    .run_events(
                        EventType::OnCommand,
                        EventData::Command(cmd.name().to_string()),
                    )
                    .await
                    .is_ok()
                {
                    // execute command
                    cmd.execute(client, args).await?;
                }
            } else {
                client.send("unknown command")?;
            }

            Ok(())
        }

        // handle errors from message processing
        if let Err(err) = handle(&client, buf).await {
            let err = err.to_string();

            // client disconnect e.g. using ctrl + c
            if err.contains("Broken pipe") {
                return Err(anyhow!("disconnected"));
            } else {
                error!("Unexpected error in message handler: {}", err);
                client.send("Unexpected error")?;
            }
        }

        client.flush()?;
    }
}

async fn start_tcp(host: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(host)?;

    let incoming = listener.incoming();

    for stream in incoming {
        let stream = stream?;

        // get id for the client
        let id = *CLIENT_NEXT.lock().unwrap();

        // add one to next id
        *CLIENT_NEXT.lock().unwrap() += 1;

        thread::spawn(move || {
            // get id for the client and add one to next id
            let client = Client::new_tcp(stream, id);

            // insert the cloned client to CLIENTS
            CLIENTS.lock().unwrap().insert(id, client.clone());

            // add span to logger
            let span = span!(Level::ERROR, "TCP", id = client.id);
            let _enter = span.enter();

            if let Err(err) = task::block_on(process(client)) {
                let err = err.to_string();

                // client disconnect e.g. using ctrl + c
                if err == "disconnected" {
                    info!("Client disconnected")
                } else {
                    error!("{}", err);
                }
            }

            // delete the client from CLIENTS map
            CLIENTS.lock().unwrap().remove(&id);
        });
    }

    Ok(())
}

async fn start_websocket(host: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(host)?;

    let incoming = listener.incoming();

    for stream in incoming {
        let stream = stream?;

        // get id for the client
        let id = *CLIENT_NEXT.lock().unwrap();

        // add one to next id
        *CLIENT_NEXT.lock().unwrap() += 1;

        thread::spawn(move || {
            let client = Client::new_websocket(stream, id).unwrap();

            // insert the cloned client to CLIENTS
            CLIENTS.lock().unwrap().insert(id, client.clone());

            // add span to logger
            let span = span!(Level::ERROR, "WS", id = client.id);
            let _enter = span.enter();

            if let Err(err) = task::block_on(process(client)) {
                let err = err.to_string();

                // client disconnect e.g. using ctrl + c
                if err == "disconnected"
                    || err.contains("Connection reset without closing handshake")
                {
                    info!("Client disconnected")
                } else {
                    error!("{}", err);
                }
            }

            // delete the client from CLIENTS map
            CLIENTS.lock().unwrap().remove(&id);
        });
    }

    Ok(())
}
