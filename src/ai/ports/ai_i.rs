use crate::ai::error::AiError;
use crate::config::AiProperties;
use std::path::PathBuf;

pub trait AiI {

    fn new(ai_settings: &AiProperties) -> Self;
    fn generate_resume(&self, path_file: &PathBuf, system_prompt: &str, user_prompt: &str) -> impl Future<Output = Result<String,AiError>>;


}
