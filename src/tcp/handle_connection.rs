use std::io::Write;

use log::trace;

use crate::plugins::{CommandManagerType, EventManagerType};

use super::Client;

/// Handle Client connection
pub async fn handle_connection(
    mut client: Client,
    commands: CommandManagerType,
    events: EventManagerType,
) -> anyhow::Result<()> {
    println!("New Client: {:?}", client.stream.peer_addr()?);

    // run `onConnect` events from plugins
    check_event(&mut client, &events, "onConnect").await;

    loop {
        // read client message/buffer
        let buf = client.read()?;

        // run `onSend` events from plugins
        check_event(&mut client, &events, "onSend").await;

        // split message by whitespaces
        let args: Vec<&str> = buf.split_ascii_whitespace().collect();

        // client sent an empty buffer
        if args.is_empty() {
            client.send("empty buffer").expect("send message");

            // don't execute the following commands because it causes panic
            continue
        }

        // get command from args
        let cmd = args[0];

        // search if a command exists
        for command in commands.commands.iter() {
            // if this is the entered command
            if cmd == command.name() {
                trace!("Executing a command `{}`", command.name());

                // execute command
                command
                    .execute(&mut client, args[1..args.len()].to_vec(), &commands)
                    .await;

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
async fn check_event(client: &mut Client, events: &EventManagerType, event_name: &str) {
    for event in events.events.iter() {
        // check if this event should be started
        if event.name() == event_name {
            trace!("Executing a event `{}`", event.name());

            // execute event
            event.execute(client).await;
        }
    }
}
