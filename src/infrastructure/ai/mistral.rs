use crate::config::AiProperties;
use crate::infrastructure::ai::error::AiError;
use crate::ports::ai_i::AiI;
use mistralai_client::v1::chat::{ChatMessage, ChatParams, ChatResponse};
use mistralai_client::v1::client::Client;
use mistralai_client::v1::constants::Model;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct MistralAdapter {
    pub client: Client,
    pub model: Model,
}

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

    async fn generate_summary(
        &self,
        path_file: &Path,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AiError> {
        let mut file = File::open(path_file)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;

        let complete_user_prompt = format!("{} \n {}", user_prompt, content);

        let assistant_message = ChatMessage::new_assistant_message(system_prompt, None);
        let user_message = ChatMessage::new_user_message(&complete_user_prompt);

        let options = ChatParams {
            temperature: 0.8,
            ..ChatParams::default()
        };

        let response_chat: ChatResponse = self
            .client
            .chat_async(
                self.model.clone(),
                vec![assistant_message, user_message],
                Some(options),
            )
            .await?;

        let mut response = String::new();

        for msg in &response_chat.choices {
            response.push_str(msg.message.content.as_str());
        }

        Ok(response)
    }
}
