use crate::plugins::prelude::*;

pub struct Disconnect;

#[async_trait]
impl Command for Disconnect {
    fn name(&self) -> &'static str {
        "/disconnect"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["/close", "/exit"]
    }

    fn help(&self) -> &'static str {
        "Close the connection"
    }

    fn usage(&self) -> &'static str {
        "/disconnect"
    }

    async fn execute(&self, client: &Client, _args: Vec<&str>) -> anyhow::Result<()> {
        // close the connection
        client.close()?;

        Ok(())
    }
}
