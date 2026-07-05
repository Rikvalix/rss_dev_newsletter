use log::{error, info};
use std::path::PathBuf;
use tldr_newsletter::ai::adapters::mistral::MistralAdapter;
use tldr_newsletter::ai::ports::ai_i::AiI;
use tldr_newsletter::config::GlobalProperties;
use tldr_newsletter::email::adapters::gmail::GmailAdapter;
use tldr_newsletter::email::ports::email::EmailI;
use tldr_newsletter::flow::ai_processor::ai_processor;
use tldr_newsletter::flow::tldr_processor::tldr_process;
use tldr_newsletter::notification::adapters::discord::DiscordAdapter;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    env_logger::init();

    let settings = match GlobalProperties::new() {
        Ok(settings) => settings,
        Err(err) => {
            error!("Fail to init configuration {}", err);
            std::process::exit(1);
        }
    };

    info!("Configuration is setup");

    let email_client: GmailAdapter = match GmailAdapter::new().await {
        Ok(a) => a,
        Err(e) => {
            error!("Fail to init the Email adapter: {}", e);
            std::process::exit(1);
        }
    };
    info!("Email client is setup");

    let ai_client = match MistralAdapter::new(&settings.ai) {
        Ok(a) => a,
        Err(err) => {
            error!("Fail to init the AI adapter: {}", err);
            std::process::exit(1);
        }
    };
    info!("AI client is setup");

    let notification_client = DiscordAdapter::new(&settings.notification.discord.webhook_url);
    // TODO: Add configuration provider to load interface instead of adapter

    info!("Run TLDR processor");
    let files: Vec<PathBuf> = match tldr_process(&email_client, &settings).await {
        Ok(a) => a,
        Err(e) => {
            error!("Error during the TLDR processor {}", e);
            std::process::exit(1);
        }
    };

    if settings.ai.enable {
        info!("Run Ai processor");
        match ai_processor(&settings.ai, &ai_client, &notification_client, &files).await {
            Ok(a) => a,
            Err(e) => {
                error!("Error during the AI processor {}", e);
                std::process::exit(1);
            }
        };
    } else {
        info!("Ai processor disabled");
    }
}
