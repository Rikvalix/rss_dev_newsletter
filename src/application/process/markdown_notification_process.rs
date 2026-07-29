use crate::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use crate::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use crate::infrastructure::database::repository::notification_repository::NotificationRepository;
use chrono::Utc;
use log::error;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct MarkdownNotificationProcess {
    pub ai_summary_repository: AiSummaryRepository,
    pub feed_item_repository: FeedItemRepository,
    pub notification_repository: NotificationRepository
}

impl MarkdownNotificationProcess {
    pub fn new(
        ai_summary_repository: &AiSummaryRepository,
        feed_item_repository: FeedItemRepository,
        notification_repository: &NotificationRepository
    ) -> MarkdownNotificationProcess {
        MarkdownNotificationProcess {
            ai_summary_repository: ai_summary_repository.clone(),
            feed_item_repository: feed_item_repository.clone(),
            notification_repository: notification_repository.clone()
        }
    }

    pub async fn process(&self) -> Result<(),> {
        let current_date = Utc::now().naive_utc().date();

        let current_summary = self
            .ai_summary_repository
            .get_by_creation_date(&current_date)
            .await
            .map_err(|err| {
                error!("Error getting summary for {}: {}", current_date, err);

            })
            .unwrap();

    }
}
