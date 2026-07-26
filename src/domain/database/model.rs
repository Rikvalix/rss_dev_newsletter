use crate::domain::ai::model::{AiClassificationResponse, AiSummaryResponse};
use crate::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use crate::infrastructure::database::repository::feed_repository::FeedRepository;
use crate::infrastructure::database::repository::notification_repository::NotificationRepository;
use crate::infrastructure::database::repository::summary_repository::SummaryRepository;
use serde::{Deserialize, Serialize};
use sqlx::types::{Json, Uuid};
use std::fmt;
use std::fmt::Formatter;
use strum_macros::EnumString;

#[derive(Debug,Clone)]
pub struct RepositoryRepoHandler {
    pub ai_classification_repository: AiClassificationRepository,
    pub ai_summary_repository: AiSummaryRepository,
    pub feed_item_repository: FeedItemRepository,
    pub feed_repository: FeedRepository,
    pub notification_repository: NotificationRepository,
    pub summary_repository: SummaryRepository,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FeedEntity {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub feed_type: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FeedItemEntity {
    pub id: i64,
    pub feed_id: i64,
    pub guid: String,
    pub url: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AiClassificationEntity {
    pub id: i64,
    pub content: Json<AiClassificationResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AiSummaryEntity {
    pub id: i64,
    pub ai_classification_id: i64,
    pub content: Json<AiSummaryResponse>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct NotificationEntity {
    pub id: i64,
    pub target: String,
    pub url: String,
    pub target_user: String,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

}
#[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotificationTarget {
    Discord,
    #[serde(other)]
    Unknown,
}
impl fmt::Display for NotificationTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NotificationTarget::Discord => write!(f, "DISCORD"),
            NotificationTarget::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct SummaryEntity {
    pub id: i64,
    pub public_id: Uuid,
    pub ai_summary_id: i64,
    pub title: String,
    pub content: String,
    pub metadata: Json<SummaryMetadata>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMetadata {
    pub model: Option<String>,
}

