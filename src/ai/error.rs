use std::fmt::Display;

type Result<T> = std::result::Result<T, AiError>;
#[derive(Debug, Clone)]
pub struct AiError {
    pub message: String,
}

impl Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AiError: {}", self.message)
    }
}