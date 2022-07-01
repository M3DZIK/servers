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
        "show help"
    }

    async fn execute(
        &self,
        client: &mut Client,
        _args: Vec<&str>,
        plugin_manager: &PluginManagerType,
    ) -> Result<()> {
        for command in plugin_manager.commands.iter() {
            client.send(&format!("{} - {}", command.name(), command.help())).await?;
        }

        Ok(())
    }
}
