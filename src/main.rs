use log::info;
use tldr_newsletter::ai::adapters::gemini::GeminiAdapter;
use tldr_newsletter::config::Settings;
use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::flow::ai_processor::ai_processor;
use tldr_newsletter::flow::tldr_processor::tldr_process;

#[tokio::main]
async fn main() {
    env_logger::init();
    let settings = Settings::new().expect("Could not load settings");
    info!("Configuration is setup");

    let email_client = GmailAdapter::new().await;
    info!("Email client is setup");

    let ai_client = GeminiAdapter::new(&settings.ai_settings);
    info!("AI client is setup");

    // TODO: Add configuration provider to load interface instead of adapter

    info!("Run TLDR processor");
    tldr_process(&email_client, &settings).await;

    info!("Run Ai processor");
    ai_processor(&ai_client, "tldr_newsletter_storage/TLDR_Dev/article_2026-05-19.md", "dev_article_2026_05_19.md").await;
}
