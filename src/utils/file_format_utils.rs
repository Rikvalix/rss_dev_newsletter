use crate::model::Sender;
use chrono::NaiveDate;

pub fn format_file_path(path: &str,date: &NaiveDate) -> String {
    format!("{}/article_{}.md",path,date)
}

pub fn format_repository_path(path: &str, sender: &Sender) -> String {
    format!("{}/{}",path,sender.to_string())
}