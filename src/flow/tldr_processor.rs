use crate::config::GlobalProperties;
use crate::email::ports::email::EmailI;
use crate::flow::error::FlowError;
use crate::model::{Email, EmailStatus};
use crate::storage::{check_or_create_folder, create_file};
use crate::utils::file_format_utils::{format_file_path, format_repository_path};
use log::{error, info};
use std::path::PathBuf;

/// Fetches unread emails and store it in the repository
///
/// # Arguments
///
/// * `client`: Email client, use to fetch email
/// * `settings`: Global configuration
///
/// returns: Vec<PathBuf, Global>
pub async fn tldr_process(
    client: &impl EmailI,
    settings: &GlobalProperties,
) -> Result<Vec<PathBuf>, FlowError> {
    info!("Starting TLDR process");
    // Fetch unread mails
    let unread_mails: Vec<Email> = client.get_unread().await.map_err(|err| {
        error!("Error getting unread email: {}", err);
        FlowError {
            message: format!("Error getting unread email: {}", err),
        }
    })?;
    let mut file_array: Vec<PathBuf> = Vec::new();

    for mail in unread_mails.iter() {
        let path = format_repository_path(&settings.storage.repository_path, &mail.sender);
        check_or_create_folder(&*path)?;
        let file_path =
            &format_file_path(&path, "article", &mail.receive_date.naive_local().date());
        let file: PathBuf = create_file(&file_path, &mail.content)?;
        file_array.push(file);
        if settings.tldr.mark_email_as_read {
            let _ = client.update_mail_status(&mail, EmailStatus::UNREAD)
                .await?;
        }
    }

    info!("Ending TLDR process ");
    Ok(file_array)
}
