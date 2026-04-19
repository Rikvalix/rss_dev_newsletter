use crate::model::{Email, EmailStatus};

pub trait EmailI {
    async fn get_unread(&self) -> Vec<Email>;
    async fn update_mail_status(&self, email: &Email, status: EmailStatus);
}
