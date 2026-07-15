use crate::domain::email::model::Sender;
use chrono::NaiveDate;

pub fn format_file_path(path: &str,file_prefix: &str, date: &NaiveDate) -> String {
    format!("{}/{}_{}.md",path,file_prefix,date)
}

pub fn format_repository_path(path: &str, sender: &Sender) -> String {
    format!("{}/{}",path,sender)
}