use async_trait::async_trait;

use crate::{
    plugins::{Command, PluginManagerType, Result},
    tcp::Client,
};

pub struct CommandHelp;

#[async_trait]
impl Command for CommandHelp {
    fn name(&self) -> &'static str {
        "/help"
    }

    fn help(&self) -> &'static str {
        "Display all available commands"
    }

    async fn execute(
        &self,
        client: &mut Client,
        _args: Vec<&str>,
        plugin_manager: &PluginManagerType,
    ) -> Result<()> {
        // Vector which will contain help messages of the commands
        let mut help = Vec::new();

        for command in plugin_manager.commands.iter() {
            // add a help message for the command
            help.push(format!("{} - {}", command.name(), command.help()));
        }

        // send help message to the client
        client.send(help.join("\n\r")).await?;

        Ok(())
    }
}
