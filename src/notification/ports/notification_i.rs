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
}
