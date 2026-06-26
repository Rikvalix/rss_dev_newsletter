use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::config::AiProperties;
use ollama_rs::Ollama;
use std::path::PathBuf;

pub struct OllamaAdapter {
    client: Ollama,
    model: String,
}

impl AiI for OllamaAdapter {
    fn new(ai_settings: &AiProperties) -> Self {
        let client = Ollama::builder().url(ai_settings.url.clone()).build();

        Self {
            client,
            model: ai_settings.model.clone(),
        }
    }

    fn generate_resume(&self, path_file: &PathBuf, system_prompt: &str, user_prompt: &str) -> Result<String, AiError>{
        todo!();
    }
}
