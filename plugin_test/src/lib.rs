use servers::{
    plugins::{Command, Event, Plugin, PluginManagerType, Registrar, Result},
    tcp::Client, async_trait,
};

struct PluginTest;

/// Create a new plugin
#[async_trait]
impl Plugin for PluginTest {
    /// Name of the plugin.
    fn name(&self) -> &'static str {
        "test"
    }

    /// A function will be executed when plugin loading.
    /// Usally used for initialization.
    async fn on_plugin_load(&self) {}
}

/// Create a new command
#[async_trait]
impl Command for PluginTest {
    /// Command name
    fn name(&self) -> &'static str {
        "/test"
    }

    /// Help message of the command
    fn help(&self) -> &'static str {
        "Test command from plugin"
    }

    /// Command function
    async fn execute(
        &self,
        client: &mut Client,
        _args: Vec<&str>,
        _commands: &PluginManagerType,
    ) -> Result<()> {
        client.send("content").await?;

        Ok(())
    }
}

/// Create a new event
#[async_trait]
impl Event for PluginTest {
    /// Event name (onConnect or onSend)
    fn name(&self) -> &'static str {
        "onConnect"
    }

    /// Event function
    async fn execute(&self, client: &mut Client) -> Result<()> {
        client
            .send(&format!("Welcome {}", client.stream.peer_addr().unwrap()))
            .await?;

        Ok(())
    }
}

/// Register plugin
#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn Registrar) {
    registrar.register_plugin(Box::new(PluginTest));
    registrar.register_command(Box::new(PluginTest));
    registrar.register_event(Box::new(PluginTest));
}
