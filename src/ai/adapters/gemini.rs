use crate::ai::error::AiError;
use crate::ai::ports::ai_i::AiI;
use crate::ai::prompt_loader::{load_ai_instruction, load_ai_message};
use crate::config::AiProperties;
use gemini_rust::client::Error;
use gemini_rust::Model::Gemini25Flash;
use gemini_rust::{FileHandle, Gemini, GenerationResponse};
use log::info;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct GeminiAdapter {
    client: Gemini,
    system_prompt_path: String,
    user_prompt_path: String,
}

impl AiI for GeminiAdapter {
    /*
       - Add function to clear files
       - Add function to count input token
    */

    async fn generate_resume(&self, path_file: &PathBuf) -> Result<String, AiError> {
        let file_handle = self.upload_file(path_file).await;

        let response: Result<GenerationResponse, Error> = self
            .client
            .generate_content()
            .with_system_prompt(load_ai_instruction(&self.system_prompt_path))
            .with_user_message_and_file(load_ai_message(&self.user_prompt_path), &file_handle)
            .unwrap_or_else(|err| panic!("Error while generating file: {}", err))
            .execute()
            .await;

        match response {
            Ok(resp) => Ok(resp.text()),
            Err(err) => Err(self.handle_error(&err)),
        }
    }
}

impl GeminiAdapter {
    pub fn new(ai_settings: &AiProperties) -> Self {
        let client = Gemini::with_model(&ai_settings.api_key, Gemini25Flash)
            .unwrap_or_else(|e| panic!("Unable to create Gemini client: {}", e));

        GeminiAdapter {
            client,
            system_prompt_path: ai_settings.system_prompt_path.clone(),
            user_prompt_path: ai_settings.user_prompt_path.clone(),
        }
    }

    pub async fn upload_file(&self, file_path: &PathBuf) -> FileHandle {
        // Extract bytes from the file
        let mut mut_file = File::open(file_path).expect("Could not open file");
        let mut bytes = Vec::new();
        mut_file
            .read_to_end(&mut bytes)
            .expect("Fail to read the file");

        let file_path_str = file_path.to_str().unwrap();

        let file_handle = self
            .client
            .create_file(bytes)
            .display_name(file_path_str.replace("/", "_"))
            .with_mime_type(
                "text/markdown"
                    .parse()
                    .unwrap_or_else(|e| panic!("Unable to create file {}: {}", &file_path_str, e)),
            )
            .upload()
            .await
            .unwrap_or_else(|e| panic!("Unable to upload file {}: {}", &file_path_str, e));

        info!("File {}: uploaded", &file_path_str);

        file_handle
    }

    pub fn handle_error(&self, err: &Error) -> AiError {
        match err {
            Error::BadResponse { code, description } =>{
                let mut description_truncate = description.as_ref().unwrap().to_owned();
                description_truncate.truncate(200);
                description_truncate.push_str("...");
                AiError {
                    message: format!("Bad response with code {}, description: {}",code, description_truncate),
                }
            } ,
            _ => AiError {
                message: format!("Unknown error: {}", err)
            },
        }
    }

    pub async fn list_all_files(&self) {}
    pub async fn clear_files() {}

    pub async fn count_token() {}
}
