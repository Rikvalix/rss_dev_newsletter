use crate::ai::error::AiError;
use std::path::PathBuf;

pub trait AiI {
    fn generate_resume(&self, path_file: &PathBuf, system_prompt: &str, user_prompt: &str) -> impl Future<Output = Result<String,AiError>>;
}
