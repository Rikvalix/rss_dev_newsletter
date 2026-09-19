use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthDto {
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsletterShortDto {
    pub public_id: String,
    pub date: NaiveDate,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsletterDto {
    pub public_id: String,
    pub date: NaiveDate,
    pub title: String,
    pub content: String,
    pub metadata: MetadataDto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataDto {
    pub model: Option<String>,
}
