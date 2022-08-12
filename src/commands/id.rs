use crate::plugins::prelude::*;

pub struct ID;

#[async_trait]
impl Command for ID {
    fn name(&self) -> &'static str {
        "/id"
    }

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn help(&self) -> &'static str {
        "Get id of the client"
    }

    fn usage(&self) -> &'static str {
        "/id"
    }

    async fn execute(&self, client: &Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(client.id)
    }
}
