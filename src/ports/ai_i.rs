use crate::config::AiProperties;
use crate::domain::ai::model::{
    AiClassificationItemResponse, AiClassificationResponse, AiSummaryResponse,
};
use crate::domain::database::model::FeedItemEntity;
use crate::infrastructure::ai::error::AiError;

/// Ai client interface to create summary from the differents emails
pub trait AiI: Sized {
    /// Create new client instance
    ///
    /// # Arguments
    ///
    /// * `ai_settings`: Configuration
    ///
    /// returns: Self
    fn new(ai_settings: &AiProperties) -> Result<Self, AiError>;

    /// Sort RSS items to establish a classification
    ///
    /// # Arguments
    ///
    /// * `feed_items`: RSS items
    ///
    /// returns: impl Future<Output=Result<ShortFeedResponse, AiError>>
    fn generate_classification(
        &self,
        items: &Vec<FeedItemEntity>,
    ) -> impl Future<Output = Result<AiClassificationResponse, AiError>>;

    fn generate_summary(
        &self,
        items: &Vec<AiClassificationItemResponse>,
    ) -> impl Future<Output = Result<AiSummaryResponse, AiError>>;
}
