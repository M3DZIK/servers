use async_trait::async_trait;
use servers::{
    plugins::{
        Command, CommandManagerType, CommandRegistrar, Event, EventRegistrar, Plugin,
        PluginRegistrar,
    },
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

#[async_trait]
impl Event for PluginTest {
    fn name(&self) -> &'static str {
        "onConnect"
    }

    async fn execute(&self, client: &mut Client) {
        client
            .send(&format!("Welcome {}", client.stream.peer_addr().unwrap()))
            .expect("send message")
    }
}

#[no_mangle]
pub fn plugin_entry(
    plugin: &mut dyn PluginRegistrar,
    command: &mut dyn CommandRegistrar,
    event: &mut dyn EventRegistrar,
) {
    // register plugin
    plugin.register(Box::new(PluginTest));
    // register command
    command.register(Box::new(PluginTest));
    // register plugin
    event.register(Box::new(PluginTest));
}
