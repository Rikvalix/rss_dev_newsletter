use crate::model::{Email, EmailStatus};

/// Email client interface for fetching and managing TLDR newsletter emails
pub trait EmailI {

    /// Fetches all unread emails
    ///
    /// returns: impl Future<Output= Vec<Email>> + Send
    fn get_unread(&self) -> impl Future<Output = Vec<Email>> + Send;

    /// Update the status of specific email
    ///
    /// # Arguments
    ///
    /// * `email`: Reference the email to update
    /// * `status`: New status to apply
    ///
    /// returns: impl Future<Output=()>+Send
    fn update_mail_status(
        &self,
        email: &Email,
        status: EmailStatus,
    ) -> impl Future<Output = ()> + Send;
}
