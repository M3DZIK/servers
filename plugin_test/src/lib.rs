use async_trait::async_trait;
use servers::{
    plugins::{Command, CommandManagerType, CommandRegistrar, Plugin, PluginRegistrar},
    tcp::Client,
};

struct PluginTest;

#[async_trait]
impl Plugin for PluginTest {
    fn name(&self) -> &'static str {
        "test"
    }

    async fn on_plugin_load(&self) {}

    async fn on_plugin_unload(&self) {}
}

#[async_trait]
impl Command for PluginTest {
    fn name(&self) -> &'static str {
        "/test"
    }

    fn help(&self) -> &'static str {
        "test command"
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>, _commands: &CommandManagerType) {
        client.send("content").expect("send message")
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn PluginRegistrar, command: &mut dyn CommandRegistrar) {
    registrar.register_plugin(Box::new(PluginTest));
    command.register_command(Box::new(PluginTest));
}
