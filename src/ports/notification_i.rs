use chrono::NaiveDate;

/// Notification trait to share status, summary
pub trait NotificationI {


    /// Send message
    ///
    /// # Arguments
    ///
    /// * `message`: str
    ///
    /// returns: impl Future<Output=Result<(), Error>>
    fn send_message(&self, message: &str) -> impl Future<Output = Result<(), reqwest::Error>>;
     fn send_summary_file(
        &self,
        date: &NaiveDate,
        user: &String,
        markdown_content: &str,
    ) -> impl Future<Output = Result<(), reqwest::Error>>;
}
