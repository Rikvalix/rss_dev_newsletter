use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::ai::prompt_loader::{load_ai_instruction, load_ai_message};
use crate::config::AiProperties;
use gemini_rust::client::Error;
use gemini_rust::{FileHandle, Gemini, GenerationResponse};
use log::{error, info};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct GeminiAdapter {
    client: Gemini,
}

impl AiI for GeminiAdapter {
    /*
       - Add function to clear files
       - Add function to count input token
    */
    fn new(ai_settings: &AiProperties) -> Result<Self, AiError> {
        info!(
            "Initializing Gemini client with model: {}",
            ai_settings.gemini.model
        );
        let client = Gemini::with_model(&ai_settings.gemini.api_key, ai_settings.gemini.model.clone())?;

        Ok(GeminiAdapter { client })
    }

    async fn generate_summary(
        &self,
        path_file: &PathBuf,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AiError> {
        let file_handle: FileHandle = self.upload_file(path_file).await.map_err(|err| err)?;

        let response: Result<GenerationResponse, Error> = self
            .client
            .generate_content()
            .with_system_prompt(load_ai_instruction(&system_prompt)?)
            .with_user_message_and_file(load_ai_message(&user_prompt)?, &file_handle)?
            .execute()
            .await;

        match response {
            Ok(resp) => Ok(resp.text()),
            Err(err) => Err(self.handle_error(&err)),
        }
    }
}

impl GeminiAdapter {
    pub async fn upload_file(&self, file_path: &PathBuf) -> Result<FileHandle, AiError> {
        // Extract bytes from the file
        let mut mut_file = File::open(file_path)?;
        let mut bytes = Vec::new();
        mut_file.read_to_end(&mut bytes)?;

        let file_path_str = file_path.to_string_lossy();

        if file_path_str.is_empty() {
            return Err(AiError {
                message: format!("Invalid file path {}",file_path.display())
            });
        }

        let file_handle: FileHandle = self
            .client
            .create_file(bytes)
            .display_name(file_path_str.replace("/", "_"))
            .with_mime_type("text/markdown".parse()?)
            .upload()
            .await
            .map_err(|err| {
                error!("File upload error {}", err.to_string());
                err
            })?;

        info!("File {}: uploaded", &file_path_str);

        Ok(file_handle)
    }

    pub fn handle_error(&self, err: &Error) -> AiError {
        match err {
            Error::BadResponse { code, description } => AiError {
                message: format!(
                    "Bad response with code {}, description: {}",
                    code,
                    description.as_ref().unwrap()
                ),
            },
            _ => AiError {
                message: format!("Unknown error: {}", err),
            },
        }
    }

    pub async fn list_all_files(&self) {}
    pub async fn clear_files() {}

    pub async fn count_token() {}
}
