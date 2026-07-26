use crate::domain::database::model::{AiSummaryEntity, FeedItemEntity};
use crate::infrastructure::ai::error::AiError;
use crate::infrastructure::ai::mistral::MistralAdapter;
use crate::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::ports::ai_i::AiI;
use log::{error, info};

#[derive(Debug)]
pub struct AiProcessor {
    pub ai_client: MistralAdapter,
    pub ai_classification_repository: AiClassificationRepository,
    pub ai_summary_repository: AiSummaryRepository
}


impl AiProcessor {

    pub fn new(ai_client: MistralAdapter, ai_classification_repository: &AiClassificationRepository, ai_summary_repository: &AiSummaryRepository) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(AiProcessor {
            ai_client,
            ai_classification_repository: ai_classification_repository.clone(),
            ai_summary_repository: ai_summary_repository.clone(),
        })
    }

    /// Generate classification and summary for RSS items
    ///
    /// # Arguments
    ///
    /// t `feed_items`: List of RSS items
    ///
    /// returns: AiSummaryEntity
    pub async fn ai_processor(
        &self,
        ai_client: &impl AiI,
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

        let classification_id: i64 = self.ai_classification_repository
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

        let summary_saved = self.ai_summary_repository
            .save(&summary, &classification_id)
            .await
            .map_err(|err| AiError {
                message: format!("Failed to save summary: {}", err),
            })?;

        Ok(summary_saved)
    }
}

