use crate::model::{Email, EmailStatus};

pub trait EmailI {
    fn get_unread(&self) -> impl Future<Output = Vec<Email>> + Send;
    fn update_mail_status(&self, email: &Email, status: EmailStatus) -> impl Future<Output = ()> + Send;
}
