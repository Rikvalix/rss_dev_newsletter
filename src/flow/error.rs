use crate::email::error::EmailError;
use std::fmt::Display;
use tokio::io;

type Result<T> = std::result::Result<T, FlowError>;

#[derive(Debug, Clone)]
pub struct FlowError {
    pub message: String,
}

impl Display for FlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlowError: {}", self.message)
    }
}

impl From<EmailError> for FlowError {
    fn from(email_error: EmailError) -> Self {
        FlowError {
            message: format!("{}", email_error),
        }
    }
}
impl From<io::Error> for FlowError {
    fn from(e: io::Error) -> Self {
        FlowError {
            message: e.to_string(),
        }
    }
}
