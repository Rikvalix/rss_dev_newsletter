use crate::domain::ai::model::{AiClassificationResponse, AiSummaryResponse};
use sqlx::types::Json;

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