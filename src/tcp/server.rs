use std::net::TcpListener;

use async_std::task;
use futures::join;
use lazy_static::lazy_static;
use tracing::{error, info};

use crate::{
    plugins::{self, manager::PluginsManagerType},
    tcp::Client,
    CLIENTS, CLIENT_NEXT,
};

pub const PLUGINS_DIR: &str = "plugins";

lazy_static! {
    pub static ref PLUGINS_MANAGER: PluginsManagerType =
        plugins::loader(PLUGINS_DIR).expect("failed to load plugins");
}

/// Start server
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

    loop {
        let buf = client.read()?;

        let mut args: Vec<&str> = buf.split_ascii_whitespace().collect();

        // if client sent an empty buffer
        if buf.is_empty() {
            client.send("empty buffer")?;
            continue;
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

        // execute command
        if let Some((_i, cmd)) = command {
            cmd.execute(&client, args).await?;
        } else {
            client.send("unknown command")?;
        }

        client.flush()?;
    }
}

async fn start_tcp(host: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(host)?;

    let incoming = listener.incoming();

    for stream in incoming {
        let stream = stream?;

        task::spawn(async {
            let client = Client::new_tcp(stream);

            // get id for the client and add one to next id
            let id = (*CLIENT_NEXT.lock().unwrap() + 1).clone();

            // insert the cloned client to CLIENTS
            CLIENTS.lock().unwrap().insert(id, client.clone());

            if let Err(err) = process(client).await {
                error!("TCP client error: {}", err);
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

        task::spawn(async {
            let client = Client::new_websocket(stream).unwrap();

            // get id for the client and add one to next id
            let id = (*CLIENT_NEXT.lock().unwrap() + 1).clone();

            // insert the cloned client to CLIENTS
            CLIENTS.lock().unwrap().insert(id, client.clone());

            if let Err(err) = process(client).await {
                error!("TCP client error: {}", err);
            }

            // delete the client from CLIENTS map
            CLIENTS.lock().unwrap().remove(&id);
        });
    }

    Ok(())
}