use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::email::ports::email::EmailI;
use tldr_newsletter::model::EmailStatus;
use tldr_newsletter::storage::{check_or_create_folder, create_file};

#[tokio::main]
async fn main() {
    // Fetch unread mails
    let client = GmailAdapter::new().await;
    let unread_mails = client.get_unread().await;

    println!("Nombres de mails non lus: {}", unread_mails.len());

    // Store it and update status
    for mail in unread_mails.iter() {
        let path = &mail.sender.to_string();
        check_or_create_folder(&path).unwrap();
        let file_path: String = format!(
            "{}/article_{}",
            path,
            mail.receive_date.naive_local().date()
        )
        .to_string();
        create_file(&file_path, &mail.content).unwrap();
        client.update_mail_status(&mail, EmailStatus::UNREAD).await;
    }
}
