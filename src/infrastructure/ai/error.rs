use gemini_rust::{ClientError, FilesError};
use mistralai_client::v1::error::{ApiError, ClientError as MistralError};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct AiError {
    pub message: String,
}

impl Display for AiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AiError: {}", self.message)
    }
}

impl From<std::io::Error> for AiError {
    fn from(err: std::io::Error) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}

impl From<serde_json::error::Error> for AiError {
    fn from(err: serde_json::error::Error) -> Self {
        AiError {
            message: format!("{}",err)
        }
    }
}

// Gemini
impl From<ClientError> for AiError {
    fn from(err: ClientError) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}
impl From<FilesError> for AiError {
    fn from(err: FilesError) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}

// Mistral

impl From<MistralError> for AiError {
    fn from(err: MistralError) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}

impl From<ApiError> for AiError {
    fn from(err: ApiError) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}

impl From<mime::FromStrError> for AiError {
    fn from(err: mime::FromStrError) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}

impl From<reqwest::Error> for AiError {
    fn from(err: reqwest::Error) -> Self {
        AiError {
            message: format!("{}", err),
        }
    }
}
