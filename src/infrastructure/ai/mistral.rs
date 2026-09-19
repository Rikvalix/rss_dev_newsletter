use crate::config::AiProperties;
use crate::domain::ai::model::{
    AiClassificationItemResponse, AiClassificationResponse, AiSummaryResponse, ShortFeedItem,
};
use crate::domain::database::model::FeedItemEntity;
use crate::infrastructure::ai::error::AiError;
use crate::ports::ai_i::AiI;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct MistralAdapter {
    pub api_key: String,
    pub endpoint: String,
    pub client: reqwest::Client,
    pub classification_agent_id: String,
    pub summary_agent_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub object: String,
    pub conversation_id: String,
    pub outputs: Vec<OutputEntry>,
    pub usage: Usage,
    pub guardrails: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutputEntry {
    pub r#object: String,
    pub r#type: String,
    pub created_at: String,
    pub completed_at: String,
    pub agent_id: String,
    pub model: String,
    pub id: String,
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
// AiI for
impl AiI for MistralAdapter {
    fn new(ai_settings: &AiProperties) -> Result<Self, AiError> {
        let mistral_properties = &ai_settings.mistral;

        Ok(MistralAdapter {
            api_key: ai_settings.mistral.api_key.clone(),
            endpoint: mistral_properties.endpoint.clone(),
            client: reqwest::Client::new(),
            classification_agent_id: ai_settings.mistral.classification_agent_id.clone(),
            summary_agent_id: ai_settings.mistral.summary_agent_id.clone(),
        })
    }

    async fn generate_classification(
        &self,
        items: &Vec<FeedItemEntity>,
    ) -> Result<AiClassificationResponse, AiError> {
        let mut items_short: Vec<ShortFeedItem> = vec![];

        for item in items.into_iter() {
            items_short.push(ShortFeedItem {
                id: item.id,
                title: item.title.clone(),
            })
        }
        let content = serde_json::to_string(&items_short)?;

        let response_chat = self
            .client
            .post(format!("{}/conversations", self.endpoint))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key.clone()))
            .json(&json!({
                  "agent_id": self.classification_agent_id,
                  "inputs": content,
            }))
            .send()
            .await?
            .json::<ConversationResponse>()
            .await?;

        let mut response = String::new();

        for msg in &response_chat.outputs {
            response.push_str(msg.clone().content.as_str());
        }

        let short_feed_response = serde_json::from_str::<AiClassificationResponse>(&response)?;

        Ok(short_feed_response)
    }

    async fn generate_summary(
        &self,
        items: &Vec<AiClassificationItemResponse>,
    ) -> Result<AiSummaryResponse, AiError> {
        let content = serde_json::to_string(&items)?;

        let response_chat = self
            .client
            .post(format!("{}/conversations", self.endpoint))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key.clone()))
            .json(&json!({
                  "agent_id": self.summary_agent_id,
                  "inputs": content,
            }))
            .send()
            .await?
            .json::<ConversationResponse>()
            .await?;

        let mut response = String::new();

        for msg in &response_chat.outputs {
            response.push_str(msg.clone().content.as_str());
        }

        let resume_response = serde_json::from_str::<AiSummaryResponse>(&response)?;

        Ok(resume_response)
    }
}
