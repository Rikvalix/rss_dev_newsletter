use chrono::NaiveDate;

/// Notification trait to share status, summary
pub trait NotificationI {


    /// Send message
    ///
    /// # Arguments
    ///
    /// * `date`: NaiveDate
    /// * `user`: String
    /// * `url_website`: String
    ///
    /// returns: impl Future<Output=Result<(), Error>>
     fn send_summary(
        &self,
        date: &NaiveDate,
        user: &String,
        url_website: &String,
    ) -> impl Future<Output = Result<(), reqwest::Error>>;
}
