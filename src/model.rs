use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Formatter;

pub struct Email {
    pub id: String,
    pub status: EmailStatus,
    pub content: String,
    pub subject: String,
    pub receive_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum EmailStatus {
    READ,
    UNREAD,
    DELETED,
}

impl fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EmailStatus::READ => f.write_str("READ"),
            EmailStatus::UNREAD => f.write_str("UNREAD"),
            EmailStatus::DELETED => f.write_str("DELETED"),
        }
    }
}
