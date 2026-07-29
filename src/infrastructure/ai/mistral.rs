use crate::config::AiProperties;
use crate::domain::ai::model::{AiClassificationItemResponse, AiClassificationResponse, AiSummaryResponse, ShortFeedItem};
use crate::domain::database::model::FeedItemEntity;
use crate::infrastructure::ai::error::AiError;
use crate::ports::ai_i::AiI;
use mistralai_client::v1::chat::{
    ChatMessage, ChatParams, ChatResponse, ResponseFormat,
};
use mistralai_client::v1::client::Client;
use mistralai_client::v1::constants::Model;

#[derive(Debug)]
pub struct MistralAdapter {
    pub client: Client,
    pub model: Model,
}

// AiI for
impl AiI for MistralAdapter {
    fn new(ai_settings: &AiProperties) -> Result<Self, AiError> {
        let mistral_properties = &ai_settings.mistral;

        let client = Client::new(
            Some(mistral_properties.api_key.clone()),
            Some(mistral_properties.endpoint.clone()),
            Some(mistral_properties.max_retries),
            Some(mistral_properties.timeout),
        )?;

        // Check if the model exists
        let model: Model =
            serde_json::from_str::<Model>(format!("\"{}\"", mistral_properties.model).as_str())?;

        Ok(MistralAdapter { client, model })
    }

    async fn generate_classification(&self, items: &Vec<FeedItemEntity>) -> Result<AiClassificationResponse, AiError> {
        let mut items_short: Vec<ShortFeedItem> = vec![];

        for item in items.into_iter() {
            items_short.push(ShortFeedItem {
                id: item.id,
                title: item.title.clone(),
            })
        }
        let content = serde_json::to_string(&items_short)?;

        let assistant_message = ChatMessage::new_user_message(content.as_str());

        let json_schema = serde_json::json!({
        "schema": {
              "type": "object",
              "title": "SimpleResponse",
              "required": [
              "important_articles"
              ],
              "properties": {
              "important_articles": {
              "type": "array",
              "items": {
                "type": "object",
                "required": [
                  "id",
                  "reason"
                ],
                "properties": {
                  "id": {
                    "type": "number",
                    "description": "Identifiant de l'\''article"
                  },
                  "reason": {
                    "type": "string",
                    "description": "Raison de la sélection de l'\''article"
                  }
                }
              },
              "maxItems": 10,
              "minItems": 2,
              "description": "Articles retenus par l'\''agent retenu comme important"
              }
              }
                      }
              });

        let options = ChatParams {
            temperature: 0.8,
            response_format: Some(ResponseFormat::json_schema(
                "response_schema",
                json_schema,
                Some(true),
            )),
            tools: None,
            ..ChatParams::default()
        };

        let response_chat: ChatResponse = self
            .client
            .chat_async(self.model.clone(), vec![assistant_message], Some(options))
            .await?;

        let mut response = String::new();

        for msg in &response_chat.choices {
            response.push_str(msg.message.content.as_str());
        }

        let short_feed_response = serde_json::from_str::<AiClassificationResponse>(&response)?;

        Ok(short_feed_response)
    }

    async fn generate_summary(&self, items: &Vec<AiClassificationItemResponse>) -> Result<AiSummaryResponse, AiError> {
        let content = serde_json::to_string(&items)?;

        let assistant_message = ChatMessage::new_user_message(content.as_str());

        let json_schema = serde_json::json!({
            "schema": {
            "type": "object",
            "required": [
              "global_title",
              "introduction",
              "sections"
            ],
            "properties": {
              "sections": {
                "type": "array",
                "items": {
                  "type": "object",
                  "required": [
                    "subtitle",
                    "source_ids",
                    "content"
                  ],
                  "properties": {
                    "content": {
                      "type": "string",
                      "description": "Le texte fluide et analytique synthétisant les articles de ce thème>"
                    },
                    "sub_title": {
                      "type": "string",
                      "description": "Titre thématique (ex: Évolution du Front-end)"
                    },
                     "source_ids": {
                      "type": "array",
                      "items": {
                        "type": "number"
                      },
                      "description": "<id_exact_1>,<id_exact_2>]"
                    }
                  }
                }
              },
              "global_title": {
                "type": "string",
                "description": "Un titre percutant qui résume la tendance principale des articles"
              },
              "introduction": {
                "type": "string",
                "description": "Une courte phrase d'\''accroche (max 30 mots) donnant le contexte global"
              }
            }
          }
        });

        let options = ChatParams {
            temperature: 0.8,
            response_format: Some(ResponseFormat::json_schema(
                "response_schema",
                json_schema,
                Some(true),
            )),
            tools: None,
            ..ChatParams::default()
        };

        let response_chat: ChatResponse = self
            .client
            .chat_async(self.model.clone(), vec![assistant_message], Some(options))
            .await?;

        let mut response = String::new();

        for msg in &response_chat.choices {
            response.push_str(msg.message.content.as_str());
        }

        let resume_response = serde_json::from_str::<AiSummaryResponse>(&response)?;

        Ok(resume_response)
    }
}
