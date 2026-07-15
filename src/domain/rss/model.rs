use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use strum_macros::EnumString;

#[derive(Debug, Default)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub feed_type: Type,
    pub is_active: bool,
}

#[derive(Debug, Default)]
pub struct FeedItem {
    pub feed_id: Option<u32>,
    pub guid: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
    Tech,
    Data,
    DevOps,
    Ai,
    #[serde(other)]
    Unknown,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Tech => write!(f, "TECH"),
            Type::Data => write!(f, "DATA"),
            Type::DevOps => write!(f, "DEVOPS"),
            Type::Ai => write!(f, "AI"),
            Type::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Tech
    }
}
