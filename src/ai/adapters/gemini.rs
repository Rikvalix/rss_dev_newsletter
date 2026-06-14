use crate::ai::ports::ai_i::AiI;
use crate::ai::prompt_loader::{load_ai_instruction, load_ai_message};
use crate::config::AiSettings;
use gemini_rust::Model::Gemini25Flash;
use gemini_rust::{FileHandle, Gemini};
use log::info;
use std::fs;

pub struct GeminiAdapter {
    client: Gemini,
    system_prompt_path: String,
    user_prompt_path: String,
}

impl AiI for GeminiAdapter {
    /*
       - Clear les fichiers ( propre à Gémini )
       - Compter les tokens du message (propre à Gémini) + logger
    */

    async fn generate_resume(&self, file_path: &str, file_name: &str) -> String {
        let file_handle = self.upload_file(file_path, file_name).await;

        let response = self
            .client
            .generate_content()
            .with_system_prompt(load_ai_instruction(&self.system_prompt_path))
            .with_user_message_and_file(
                load_ai_message(&self.user_prompt_path),
                &file_handle,
            )
            .unwrap_or_else(|err| panic!("Error while generating file: {}", err))
            .execute()
            .await
            .unwrap_or_else(|err| panic!("Failed to generate resume: {}", err));

        info!(
            "Response: \nId: {:?} \nContent: {:?}",
            response.response_id,
            response.text()
        );

        response.text()
    }
}

impl GeminiAdapter {
    pub fn new(ai_settings: &AiSettings) -> Self {
        let client = Gemini::with_model(&ai_settings.api_key, Gemini25Flash)
            .unwrap_or_else(|e| panic!("Unable to create Gemini client: {}", e));

        GeminiAdapter {
            client,
            system_prompt_path: ai_settings.system_prompt_path.clone(),
            user_prompt_path: ai_settings.user_prompt_path.clone(),
        }
    }

    pub async fn upload_file(&self, file_path: &str, filename: &str) -> FileHandle {
        let file = self
            .client
            .create_file(
                fs::read(&file_path)
                    .unwrap_or_else(|e| panic!("Unable to create file {}: {}", &file_path, e)),
            )
            .display_name(filename)
            .with_mime_type(
                "text/markdown"
                    .parse()
                    .unwrap_or_else(|e| panic!("Unable to create file {}: {}", &file_path, e)),
            )
            .upload()
            .await
            .unwrap_or_else(|e| panic!("Unable to upload file {}: {}", &file_path, e));

        info!("File {}: uploaded", &file_path);

        file
    }

    pub async fn clear_files() {}

    pub async fn count_token() {}
}
