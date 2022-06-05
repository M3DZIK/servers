use std::io::Write;

use log::trace;

use crate::plugins::CommandManagerType;

use super::Client;

/// Handle Client connection
pub async fn handle_connection(
    mut client: Client,
    commands: CommandManagerType,
) -> anyhow::Result<()> {
    println!("New Client: {:?}", client.stream.peer_addr()?);

    loop {
        // read client message/buffer
        let buf = client.read().await?;

        // split message by whitespace
        let args: &Vec<&str> = &buf.split_ascii_whitespace().collect();

        // get command from args
        let cmd = args[0];

        // search if a command exists
        for command in commands.commands.iter() {
            // if this is the entered command
            if cmd == command.name() {
                // execute command
                trace!("Executing a command `{}`", command.name());
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
