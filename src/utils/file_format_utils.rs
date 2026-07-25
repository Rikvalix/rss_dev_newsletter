use chrono::NaiveDate;

pub fn format_file_path(path: &str, file_prefix: &str, date: &NaiveDate) -> String {
    format!("{}/{}_{}.md", path, file_prefix, date)
}
