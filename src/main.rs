use servers::{loader, Client, PluginManagerType, register_commands, CommandManagerType};
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // listen Tcp server
    let listener = TcpListener::bind("0.0.0.0:9999").await?;

    // load plugins
    let plugin_manager = loader()?;

    // load command
    let commands_manager = register_commands();

    // Accepts a new incoming connection from this listener.
    while let Ok((stream, _address)) = listener.accept().await {
        let client = Client::new(stream);

        // handle client connection in new thread
        tokio::spawn(handle_connection(client, (*plugin_manager).to_vec(), (*commands_manager).to_vec()));
    }

    Ok(())
}

async fn handle_connection(mut client: Client, plugins: PluginManagerType, commands: CommandManagerType) -> anyhow::Result<()> {
    println!("New Client: {:?}", client.stream.peer_addr()?);

    loop {
        // read client message/buffer
        let buf = client.read().await?;

        // split message by whitespace
        let args: &Vec<&str> = &buf.split_ascii_whitespace().collect();

        // get command from args
        let cmd = args[0];

        println!("{:?}", &args);

        for command in &commands {
            if cmd == command.name() {
                println!("s");
                command.execute(&mut client, args[1..args.len()].to_vec(), &commands).await;
                break
            }
        }

        // search command in plugins
        for plugin in &plugins {
            // if command found execute plugin
            if cmd == plugin.command() {
                plugin.execute(&mut client, args[1..args.len()].to_vec())
            }
        }

        // if an I/O or EOF error, abort the connection
        if client.stream.flush().await.is_err() {
            break;
        }
    }

    Ok(())
}
