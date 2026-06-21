use crate::config::GlobalProperties;
use crate::email::ports::email::EmailI;
use crate::model::EmailStatus;
use crate::storage::{check_or_create_folder, create_file};
use crate::utils::file_format_utils::{format_file_path, format_repository_path};
use log::info;
use std::path::PathBuf;

pub async fn tldr_process(client: &impl EmailI, settings: &GlobalProperties) -> Vec<PathBuf> {
    info!("Starting TLDR process");
    // Fetch unread mails
    let unread_mails = client.get_unread().await;
    let mut file_array : Vec<PathBuf> = Vec::new();

    for mail in unread_mails.iter() {
        let path = format_repository_path(&settings.storage.repository_path, &mail.sender);
        check_or_create_folder(&*path).unwrap();
        let file = create_file(
            &format_file_path(&path, "article",&mail.receive_date.naive_local().date()),
            &mail.content,
        );
        file_array.push(file);
        if settings.tldr.mark_email_as_read {
            client.update_mail_status(&mail, EmailStatus::UNREAD).await;
        }
    }

    info!("Ending TLDR process ");
    file_array
}
