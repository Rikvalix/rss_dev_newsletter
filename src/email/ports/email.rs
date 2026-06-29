use crate::email::error::EmailError;
use crate::model::{Email, EmailStatus};

/// Email client interface for fetching and managing TLDR newsletter emails
pub trait EmailI: Sized {

    /// Create new email client instance
    ///
    /// returns: impl Future<Output = Result<Self, EmailError>>
    fn new() -> impl Future<Output = Result<Self, EmailError>>;

    /// Fetches all unread emails
    ///
    /// returns: impl Future<Output= Result<Vec<Email>, EmailError>> + Send
    fn get_unread(&self) -> impl Future<Output = Result<Vec<Email>, EmailError>> + Send;

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
    ) -> impl Future<Output = Result<(), EmailError>> + Send;
}
