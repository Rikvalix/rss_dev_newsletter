use crate::domain::database::model::{AiSummaryEntity, FeedItemEntity};
use crate::infrastructure::ai::error::AiError;
use crate::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::ports::ai_i::AiI;
use log::{error, info};

/// Generate classification and summary for RSS items
///
/// # Arguments
///
/// * `ai_client`: Ai client, use to generate summary
/// * `ai_classification_repository`: Ai classification repository
/// * `ai_summary_repository` : AI Summary repository
/// t `feed_items`: List of RSS items
///
/// returns: AiSummaryEntity
pub async fn ai_processor(
    ai_client: &impl AiI,
    ai_classification_repository: &AiClassificationRepository,
    ai_summary_repository: &AiSummaryRepository,
    feed_items: &Vec<FeedItemEntity>,
) -> Result<AiSummaryEntity, AiError> {
    info!("Starting AI processor");

    if feed_items.is_empty() {
        info!("No items to process");
    }
    
    let classified_items = ai_client
        .generate_classification(&feed_items)
        .await
        .map_err(|err| {
            error!("Failed to generate classification: {}", err);
        })
        .unwrap();

    let classification_id: i64 = ai_classification_repository
        .save(&classified_items)
        .await
        .map_err(|err| AiError {
            message: format!("Failed to save classification: {}", err),
        })?
        .id;

    let summary = ai_client
        .generate_summary(&classified_items.important_articles)
        .await
        .map_err(|err| {
            error!("Failed to generate summary: {}", err);
        })
        .unwrap();

    let summary_saved = ai_summary_repository
        .save(&summary, &classification_id)
        .await
        .map_err(|err| AiError {
            message: format!("Failed to save summary: {}", err),
        })?;

    Ok(summary_saved)
}
