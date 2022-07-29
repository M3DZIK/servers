use async_trait::async_trait;
use tokio::io::AsyncWriteExt;

use crate::{
    plugins::{Command, PluginManagerType, Result},
    tcp::Client,
};

pub struct CommandDisconnect;

#[async_trait]
impl Command for CommandDisconnect {
    fn name(&self) -> &'static str {
        "/disconnect"
    }

    fn help(&self) -> &'static str {
        "Disconnect from the server"
    }

    async fn execute(
        &self,
        client: &mut Client,
        _args: Vec<&str>,
        _plugin_manager: &PluginManagerType,
    ) -> Result<()> {
        // close the connection
        client.stream.shutdown().await?;

        Ok(())
    }
}
