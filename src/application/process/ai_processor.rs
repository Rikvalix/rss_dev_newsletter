use crate::domain::database::model::{AiClassificationEntity, AiSummaryWithFeedItems};
use crate::infrastructure::ai::error::AiError;
use crate::infrastructure::ai::mistral::MistralAdapter;
use crate::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use crate::ports::ai_i::AiI;
use chrono::{NaiveDate, Utc};
use log::{error, info};
use std::error::Error;

#[derive(Debug)]
pub struct AiProcessor {
    pub ai_client: MistralAdapter,
    pub ai_classification_repository: AiClassificationRepository,
    pub ai_summary_repository: AiSummaryRepository,
    pub feed_item_repository: FeedItemRepository,
}

impl AiProcessor {
    pub fn new(
        ai_client: MistralAdapter,
        ai_classification_repository: &AiClassificationRepository,
        ai_summary_repository: &AiSummaryRepository,
        feed_item_repository: &FeedItemRepository,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(AiProcessor {
            ai_client,
            ai_classification_repository: ai_classification_repository.clone(),
            ai_summary_repository: ai_summary_repository.clone(),
            feed_item_repository: feed_item_repository.clone(),
        })
    }

    pub async fn process(&self) -> Result<(), AiError> {
        let current_date: NaiveDate = Utc::now().naive_utc().date();
        let check_classification_already_generated: Option<AiClassificationEntity> = self
            .ai_classification_repository
            .get_by_creation_date(&current_date)
            .await
            .map_err(|err| {
                error!(
                    "Fail to check if classification has been generated for {} : {}",
                    &current_date, err
                )
            })
            .unwrap();

        if check_classification_already_generated.is_none() {
            self.process_classification().await?;
        } else {
            info!("AI classification has been already proceed");
        }

        let check_summary_generated: Option<AiSummaryWithFeedItems> = self
            .ai_summary_repository
            .get_by_creation_date_with_feed_items(&current_date)
            .await
            .map_err(|err| {
                error!(
                    "Fail to check if classification has been generated for {} : {}",
                    &current_date, err
                )
            })
            .unwrap();

        if check_summary_generated.is_none() {
            self.process_summary(&current_date).await?;
        } else {
            info!("AI summary has been already proceed")
        }
        Ok(())
    }

    /// Generate classification  RSS items
    async fn process_classification(&self) -> Result<(), AiError> {
        let today_feeds = self
            .feed_item_repository
            .find_by_creation_date(&Utc::now().date_naive())
            .await
            .map_err(|err| error!("Fail to find feeds items: {}", err))
            .unwrap();

        if today_feeds.is_empty() {
            info!("No items to process");
            return Ok(());
        }

        let classified_items = self
            .ai_client
            .generate_classification(&today_feeds)
            .await
            .map_err(|err| {
                error!("Failed to generate classification: {}", err);
            })
            .unwrap();

        let mut selected_items_id: Vec<i64> = vec![];

        classified_items.important_articles.iter().for_each(|item| {
            if let Some(item) = today_feeds
                .iter()
                .filter(|today_item| today_item.id == item.id)
                .next()
            {
                selected_items_id.push(item.id)
            }
        });

        self.ai_classification_repository
            .save(&classified_items, &selected_items_id)
            .await
            .map_err(|err| AiError {
                message: format!("Failed to save classification: {}", err),
            })?
            .id;

        Ok(())
    }

    /// Generate AI summary and send notification
    pub async fn process_summary(&self, date: &NaiveDate) -> Result<(), AiError> {
        let classification_entity_opt = self
            .ai_classification_repository
            .get_by_creation_date(date)
            .await
            .map_err(|err| {
                error!(
                    "Fail to get AI classification entity for date {} : {}",
                    &date, err
                )
            })
            .unwrap();

        if classification_entity_opt.is_none() {
            return Ok(());
        }

        let classified_entity = classification_entity_opt.unwrap();

        let summary = self
            .ai_client
            .generate_summary(&classified_entity.content.important_articles)
            .await
            .map_err(|err| {
                error!("Failed to generate summary: {}", err);
            })
            .unwrap();

        self.ai_summary_repository
            .save(&summary, &classified_entity.id)
            .await
            .map_err(|err| AiError {
                message: format!("Failed to save summary: {}", err),
            })?;

        Ok(())
    }
}
