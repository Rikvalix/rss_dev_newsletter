use crate::domain::database::model::{AiSummaryWithFeedItems, SummaryMetadata};
use crate::infrastructure::ai::error::AiError;
use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use crate::infrastructure::database::repository::notification_repository::NotificationRepository;
use crate::infrastructure::database::repository::summary_repository::SummaryRepository;
use crate::infrastructure::markdown_generator::generator::summary_generator;
use crate::infrastructure::notification::discord::DiscordAdapter;
use crate::ports::notification_i::NotificationI;
use chrono::Utc;
use log::error;
use std::error::Error;

#[derive(Debug)]
pub struct MarkdownNotificationProcessor {
    pub ai_summary_repository: AiSummaryRepository,
    pub feed_item_repository: FeedItemRepository,
    pub notification_repository: NotificationRepository,
    pub summary_repository: SummaryRepository,
    pub website: String
}

impl MarkdownNotificationProcessor {
    pub fn new(
        ai_summary_repository: &AiSummaryRepository,
        feed_item_repository: &FeedItemRepository,
        notification_repository: &NotificationRepository,
        summary_repository: &SummaryRepository,
        website: &String
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            ai_summary_repository: ai_summary_repository.clone(),
            feed_item_repository: feed_item_repository.clone(),
            notification_repository: notification_repository.clone(),
            summary_repository: summary_repository.clone(),
            website: website.clone()
        })
    }

    pub async fn process(&self) -> Result<(), AiError> {
        let current_date = Utc::now().naive_utc().date();

        let current_summary_items: Option<AiSummaryWithFeedItems> = self
            .ai_summary_repository
            .get_by_creation_date_with_feed_items(&current_date)
            .await
            .map_err(|err| {
                error!("Error getting summary for {}: {}", current_date, err);
            })
            .unwrap();

        if current_summary_items.is_none() {
            return Ok(());
        }

        let current_summary_items: AiSummaryWithFeedItems = current_summary_items.unwrap();

        let markdown_gen = summary_generator(
            &current_summary_items.ai_summary.content,
            &current_summary_items.feed_items,
        );

        let markdown_entity = self
            .summary_repository
            .save(
                &current_summary_items.ai_summary,
                &markdown_gen,
                &SummaryMetadata {
                    model: Some("mistral-small-latest".to_string()),
                },
            )
            .await
            .unwrap();

        let target_users = self.notification_repository.get_all(true).await.unwrap();

        for user in target_users {
            let discord = DiscordAdapter::new(&user.url);
            discord
                .unwrap()
                .send_summary(
                    &current_date, 
                    &user.target_user,
                     &format!("{}/{}",self.website,markdown_entity.public_id))
                .await?;
        }
        Ok(())
    }
}
