use crate::flow::error::FlowError;
use std::fmt::Display;
use std::io::Error;

type Result<T> = std::result::Result<T, EmailError>;

#[derive(Debug, Clone)]
pub struct EmailError {
    pub message: String,
}

impl Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmailError: {}", self.message)
    }
}

impl From<FlowError> for EmailError {
    fn from(flow_error: FlowError) -> Self {
        EmailError {
            message: flow_error.message,
        }
    }
}
impl From<Error> for EmailError {
    fn from(value: Error) -> Self {
        EmailError {
            message: format!("{}", value),
        }
    }
}

impl From<regex::Error> for EmailError {
    fn from(value: regex::Error) -> Self {
        EmailError {
            message: format!("{}", value),
        }
    }
}

impl From<google_gmail1::Error> for EmailError {
    fn from(error: google_gmail1::Error) -> Self {
        EmailError {
            message: format!("{}", error),
        }
    }
}

impl From<google_gmail1::yup_oauth2::Error> for EmailError {
    fn from(error: google_gmail1::yup_oauth2::Error) -> Self {
        EmailError {
            message: format!("{}", error),
        }
    }
}
