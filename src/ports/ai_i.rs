use crate::config::AiProperties;
use crate::infrastructure::ai::error::AiError;
use std::path::Path;

/// Ai client interface to create summary from the differents emails
pub trait AiI: Sized {
    /// Create new client instance
    ///
    /// # Arguments
    ///
    /// * `ai_settings`: Configuration
    ///
    /// returns: Self
    fn new(ai_settings: &AiProperties) -> Result<Self, AiError>;

    /// Generate summary for specific file
    ///
    /// # Arguments
    ///
    /// * `path_file`: File with the content to summary
    /// * `system_prompt`: Prompt system
    /// * `user_prompt`: User prompt
    ///
    /// returns: impl Future<Output=Result<String, AiError>>
    fn generate_summary(
        &self,
        path_file: &Path,
        system_prompt: &str,
        user_prompt: &str,
    ) -> impl Future<Output = Result<String, AiError>>;
}
