use crate::ports::notification_i::NotificationI;
use serde_json::json;

pub struct DiscordAdapter {
    client: reqwest::Client,
    base_url: String,
}

impl NotificationI for DiscordAdapter {
    // Send message on the discord webhook, split it if the len is more than 2000
    async fn send_message(&self, message: &str) -> Result<(), reqwest::Error> {
        if message.chars().count() <= 2000 {
            self.send(message).await?;
        } else {
            let mut current_chunk = String::new();
            let mut char_count = 0;

            for c in message.chars() {
                if char_count == 2000 {
                    self.send(&current_chunk).await?;
                    current_chunk.clear();
                    char_count = 0;
                }
                current_chunk.push(c);
                char_count += 1;
            }

            if !current_chunk.is_empty() {
                self.send(&current_chunk).await?;
            }
        }
        Ok(())
    }
}

impl DiscordAdapter {
    pub fn new(webhook_url: &str) -> Result<Self,Box<dyn std::error::Error>> {
       let client =  DiscordAdapter {
            client: reqwest::Client::new(),
            base_url: webhook_url.to_string(),
        };
        Ok(client)
    }

    async fn send(&self, message: &str) -> Result<(), reqwest::Error> {
        let payload = json!({
            "content": message.to_string(),
        });

        self.client
            .post(self.base_url.clone())
            .json(&payload)
            .send()
            .await?;

        Ok(())
    }
}
