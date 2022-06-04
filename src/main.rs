use servers::{loader, Client, CommandManagerType};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind("0.0.0.0:9999").await?;

    println!("Tcp server started at: {}", listener.local_addr()?);

    // load plugins and commands
    let (_plugin_manager, commands_manager) = loader()?;

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept().await {
        let client = Client::new(stream);

        // handle client connection in new thread
        tokio::spawn(handle_connection(client, commands_manager.clone()));
    }

    Ok(())
}

async fn handle_connection(mut client: Client, commands: CommandManagerType) -> anyhow::Result<()> {
    println!("New Client: {:?}", client.stream.peer_addr()?);

    loop {
        // read client message/buffer
        let buf = client.read().await?;

        // split message by whitespace
        let args: &Vec<&str> = &buf.split_ascii_whitespace().collect();

        // get command from args
        let cmd = args[0];

        // search if a command exists
        for command in commands.iter() {
            // if this is the entered command
            if cmd == command.name() {
                // execute command
                command
                    .execute(&mut client, args[1..args.len()].to_vec(), &commands)
                    .await;

                // don't search for more commands
                break;
            }
        }

        // if an I/O or EOF error, abort the connection
        if client.stream.flush().await.is_err() {
            // terminate connection
            break;
        }
    }

    Ok(())
}
