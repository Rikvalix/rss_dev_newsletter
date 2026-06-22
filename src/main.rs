use log::info;
use std::path::PathBuf;
use tldr_newsletter::ai::adapters::gemini::GeminiAdapter;
use tldr_newsletter::ai::ports::ai_i::AiI;
use tldr_newsletter::config::GlobalProperties;
use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::flow::ai_processor::ai_processor;
use tldr_newsletter::flow::tldr_processor::tldr_process;
use tldr_newsletter::notification::adapters::discord::DiscordAdapter;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    env_logger::init();
    let settings = GlobalProperties::new().expect("Could not load settings");
    info!("Configuration is setup");

    let email_client = GmailAdapter::new().await;
    info!("Email client is setup");

    let ai_client = GeminiAdapter::new(&settings.ai);
    info!("AI client is setup");

    let discord_client = DiscordAdapter::new(&settings.notification.discord.webhook_url);
    // TODO: Add configuration provider to load interface instead of adapter

    info!("Run TLDR processor");
    let files: Vec<PathBuf> = tldr_process(&email_client, &settings).await;

    if settings.ai.enable {
        info!("Run Ai processor");
        ai_processor(&settings.ai, &ai_client, &discord_client, &files).await;
    } else {
        info!("Ai processor disabled");
    }
}
