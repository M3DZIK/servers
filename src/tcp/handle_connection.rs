use std::io::Write;

use log::{trace, error};

use crate::plugins::PluginManagerType;

use super::Client;

/// Handle Client connection
pub async fn handle_connection(
    mut client: Client,
    plugin_manager: PluginManagerType,
) -> anyhow::Result<()> {
    println!("New Client: {:?}", client.stream.peer_addr()?);

    // run `onConnect` events from plugins
    check_event(&mut client, &plugin_manager, "onConnect").await;

    loop {
        // read client message/buffer
        let buf = client.read()?;

        // run `onSend` events from plugins
        check_event(&mut client, &plugin_manager, "onSend").await;

        // split message by whitespaces
        let args: Vec<&str> = buf.split_ascii_whitespace().collect();

        // client sent an empty buffer
        if args.is_empty() {
            client.send("empty buffer").expect("send message");

            // don't execute the following commands because it causes panic
            continue;
        }

        // get command from args
        let cmd = args[0];

        // search if a command exists
        for command in plugin_manager.commands.iter() {
            // if this is the entered command
            if cmd == command.name() {
                trace!("Executing a command `{}`", command.name());

                // execute command
                let out = command
                    .execute(&mut client, args[1..args.len()].to_vec(), &plugin_manager)
                    .await;

                match out {
                    Ok(_) => (),
                    Err(err) => {
                        error!("failed to execute command `{cmd}`, error message = `{err}`");

                        client.send(&format!("error: {err}")).expect("send message to client");
                    },
                }

                // don't search for more commands
                break;
            }
        }

        // if an I/O or EOF error, abort the connection
        if client.stream.flush().is_err() {
            // terminate connection
            break;
        }
    }

    Ok(())
}

/// Search for a events and execute it
async fn check_event(client: &mut Client, events: &PluginManagerType, event_name: &str) {
    for event in events.events.iter() {
        // check if this event should be started
        if event.name() == event_name {
            trace!("Executing a event `{}`", event.name());

            // execute event
            let out = event.execute(client).await;

            match out {
                Ok(_) => (),
                Err(err) => {
                    error!("failed to execute event `{event_name}`, error message = `{err}`");

                    client.send(&format!("error: {err}")).expect("send message to client");
                },
            }
        }
    }
}
