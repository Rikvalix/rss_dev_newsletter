use crate::config::Settings;
use crate::email::ports::email::EmailI;
use crate::model::EmailStatus;
use crate::storage::{check_or_create_folder, create_file};
use crate::utils::file_format_utils::{format_file_path, format_repository_path};
use log::info;

pub async fn tldr_process(client: &impl EmailI, settings: &Settings) {
    info!("Starting TLDR process");
    // Fetch unread mails
    let unread_mails = client.get_unread().await;

    info!("Unread mails: {:?}", unread_mails);

    for mail in unread_mails.iter() {
        let path = format_repository_path(&settings.storage_settings.repository_path, &mail.sender);
        check_or_create_folder(&*path).unwrap();
        create_file(
            &format_file_path(&path, &mail.receive_date.naive_local().date()),
            &mail.content,
        )
        .unwrap();
        client.update_mail_status(&mail, EmailStatus::UNREAD).await;
    }

    info!("Ending TLDR process ")
}
