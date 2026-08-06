use crate::ports::notification_i::NotificationI;
use chrono::NaiveDate;
use reqwest::multipart::{Form};
use serde_json::json;

pub struct DiscordAdapter {
    client: reqwest::Client,
    base_url: String,
}

impl NotificationI for DiscordAdapter {
    
    async fn send_summary(
        &self,
        date: &NaiveDate,
        user: &String,
        url_website: &String,
    ) -> Result<(), reqwest::Error> {
        
        let payload_json = json!({
            "content": format!("**Bonjour {}, le brief IA du {}** est disponible à ce lien {} !*",user,date,url_website),
            "username": "Bot Aggrégateur RSS"
        })
            .to_string();

        let form = Form::new()
            .text("payload_json", payload_json);

        let response = self
            .client
            .post(&self.base_url)
            .multipart(form)
            .send()
            .await?;

        response.error_for_status()?;

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
}
