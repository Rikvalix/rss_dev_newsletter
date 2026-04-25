use log::info;
use tldr_newsletter::config::Settings;
use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::flow::tldr_processor::tldr_process;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Could not load settings");
    info!("Configuration is setup");

    let client = GmailAdapter::new().await;
    info!("Email client is setup");

    info!("Run TLDR processor");
    tldr_process(&client, &settings).await
}
