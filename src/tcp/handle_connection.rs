use std::io::Write;

use log::{error, trace};

use crate::plugins::PluginManagerType;

use super::Client;

/// Handle Client connection
pub async fn handle_connection(
    mut client: Client,
    plugin_manager: PluginManagerType,
) -> anyhow::Result<()> {
    println!("New Client: {}", client.stream.peer_addr()?);

    // run `onConnect` events from plugins
    check_event(&mut client, &plugin_manager, "onConnect").await?;

    loop {
        // read client message/buffer
        let buf = client.read()?;

        // run `onSend` events from plugins
        check_event(&mut client, &plugin_manager, "onSend").await?;

        // split the message by whitespaces and collect it into Vec<&str>
        let mut args = buf.split_ascii_whitespace().collect::<Vec<&str>>();

        // client sent an empty buffer
        if args.is_empty() {
            client.send("empty buffer")?;

            // don't execute the following commands because it causes panic
            continue;
        }

        // get command from args
        let cmd = args[0];

        // remove command name from args
        args = args[1..args.len()].to_vec();

        // search if a command exists
        for command in plugin_manager.commands.iter() {
            // if this is the entered command
            if cmd == command.name() {
                trace!("Executing a command `{}`", command.name());

                // execute command
                match command.execute(&mut client, args, &plugin_manager).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!("failed to execute command `{cmd}`, error message = `{err}`");

                        client.send(&format!("error: {err}"))?;
                    }
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
async fn check_event(
    client: &mut Client,
    events: &PluginManagerType,
    event_name: &str,
) -> anyhow::Result<()> {
    for event in events.events.iter() {
        // check if this event should be started
        if event.name() == event_name {
            trace!("Executing a event `{}`", event.name());

            // execute event
            match event.execute(client).await {
                Ok(_) => (),
                Err(err) => {
                    error!("failed to execute event `{event_name}`, error message = `{err}`");

                    client.send(&format!("error: {err}"))?;
                }
            }
        }
    }

    Ok(())
}
