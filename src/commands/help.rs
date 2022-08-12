use crate::plugins::prelude::*;

pub struct Help;

#[async_trait]
impl Command for Help {
    fn name(&self) -> &'static str {
        "/help"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["/h", "/?", "?"]
    }

    fn help(&self) -> &'static str {
        "Show commands help menu"
    }

    fn usage(&self) -> &'static str {
        "/help"
    }

    async fn execute(&self, client: &Client, _args: Vec<&str>) -> anyhow::Result<()> {
        let mut msg = Vec::new();

        for cmd in client.plugins_manager.commands.iter() {
            let aliases = cmd.aliases();

            let aliases = if !aliases.is_empty() {
                cmd.aliases().join(", ")
            } else {
                "none".to_string()
            };

            msg.push(format!(
                "{name} - {help} (Aliases: {aliases})",
                name = cmd.name(),
                help = cmd.help(),
                aliases = aliases,
            ))
        }

        client.send(msg.join("\n"))?;

        Ok(())
    }
}
