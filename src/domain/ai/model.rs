use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortFeedItem {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiClassificationResponse {
    pub important_articles: Vec<AiClassificationItemResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AiClassificationItemResponse {
    pub id: i64,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiSummaryResponse {
    pub global_title: String,
    pub introduction: String,
    pub sections: Vec<AiSummaryContentItems>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiSummaryContentItems {
    pub sub_title: String,
    pub source_ids: Vec<i64>,
    pub content: String
}
