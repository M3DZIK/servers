use async_std::task;

use crate::{plugins::prelude::*, CLIENTS};

pub struct Broadcast;

#[async_trait]
impl Command for Broadcast {
    fn name(&self) -> &'static str {
        "/broadcast"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec![]
    }

    fn help(&self) -> &'static str {
        "Send message to all connected clients"
    }

    fn usage(&self) -> &'static str {
        "/broadcast <message>"
    }

    async fn execute(&self, client: &Client, args: Vec<&str>) -> anyhow::Result<()> {
        if args.is_empty() || args.join(" ").is_empty() {
            client.send("Missing message")?;
            return Ok(());
        }

        let msg = args.join(" ");

        let mut children = Vec::new();

        // send message to all connected clients
        for (_i, client) in CLIENTS.lock().unwrap().clone() {
            let msg = msg.clone();
            let child = task::spawn(async move {
                client
                    .send(msg)
                    .expect("failed to send broadcast message to client")
            });

            children.push(child);
        }

        // wait for all task to complete
        for child in children {
            child.await;
        }

        Ok(())
    }
}
