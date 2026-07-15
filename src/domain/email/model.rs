use chrono::{DateTime, Utc};
use std::fmt;
use std::fmt::Formatter;
use strum_macros::EnumString;

#[derive(Debug)]
pub struct Email {
    pub id: String,
    pub status: EmailStatus,
    pub content: String,
    pub sender: Sender,
    pub receive_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, EnumString)]
pub enum Sender {
    #[strum(serialize = "TLDR")]
    Tldr,
    #[strum(serialize = "TLDR AI")]
    TldrAi,
    #[strum(serialize = "TLDR IT")]
    TldrIt,
    #[strum(serialize = "TLDR InfoSec")]
    TldrInfoSec,
    #[strum(serialize = "TLDR Dev")]
    TldrDev,
    #[strum(serialize = "TLDR DevOps")]
    TldrDevOps,
    #[strum(serialize = "TLDR Data")]
    TldrData,
    #[strum(serialize = "To Sort")]
    ToSort,
}
impl fmt::Display for Sender {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sender::Tldr => write!(f, "TLDR"),
            Sender::TldrAi => write!(f, "TLDR_AI"),
            Sender::TldrIt => write!(f, "TLDR_IT"),
            Sender::TldrInfoSec => write!(f, "TLDR_InfoSec"),
            Sender::TldrDev => write!(f, "TLDR_Dev"),
            Sender::TldrDevOps => write!(f, "TLDR_DevOps"),
            Sender::TldrData => write!(f, "TLDR_Data"),
            Sender::ToSort => write!(f, "To_Sort"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EmailStatus {
    UNREAD,
    DELETED,
}

impl fmt::Display for EmailStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EmailStatus::UNREAD => f.write_str("UNREAD"),
            EmailStatus::DELETED => f.write_str("DELETED"),
        }
    }
}
