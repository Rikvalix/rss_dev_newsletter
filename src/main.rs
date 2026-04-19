use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::email::ports::email::EmailI;
use tldr_newsletter::model::{Email, EmailStatus};

#[tokio::main]
async fn main() {
    let client = GmailAdapter::new().await;
    let result = client.get_unread().await;

    for val in result.iter() {
        println!("{:?}", val.id);
    }
    println!("Nombres de mails non lus: {}", result.len());

    let first_result: &Email = result.first().unwrap();
    let update_result = client
        .update_mail_status(first_result, EmailStatus::UNREAD)
        .await;
}
