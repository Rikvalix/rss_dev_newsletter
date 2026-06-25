pub trait NotificationI {
    fn send_message(&self, message: &str) -> impl Future<Output = Result<(), reqwest::Error>>;
}
