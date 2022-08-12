use servers::plugins::prelude::*;

struct PluginTest;

#[async_trait]
impl Plugin for PluginTest {
    /// Name of the plugin.
    fn name(&self) -> &'static str {
        "test_plugin"
    }
    /// A function that will be executed when the plugin is loaded.
    async fn on_load(&self) {}
}

#[async_trait]
impl Command for PluginTest {
    /// Name of the command.
    fn name(&self) -> &'static str {
        "/test"
    }
    /// Aliases for the command.
    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }
    /// Help message of the command.
    fn help(&self) -> &'static str {
        "Test commend loaded from dylib"
    }
    /// Usage message of the command.
    fn usage(&self) -> &'static str {
        "/test"
    }
    /// Command function.
    async fn execute(&self, client: &Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send("successful executed command from dylib")?;

        Ok(())
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn Registrar) {
    registrar.register_plugins(Box::new(PluginTest));
    registrar.register_commands(Box::new(PluginTest));
}
