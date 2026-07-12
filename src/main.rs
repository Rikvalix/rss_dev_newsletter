use log::{error, info};
use std::path::PathBuf;
use tldr_newsletter::application::ai_processor::ai_processor;
use tldr_newsletter::application::rss_processor::rss_processor;
use tldr_newsletter::application::tldr_processor::tldr_process;
use tldr_newsletter::config::GlobalProperties;
use tldr_newsletter::domain::database::model::FeedEntity;
use tldr_newsletter::domain::rss::model::Feed;
use tldr_newsletter::infrastructure::ai::gemini::GeminiAdapter;
use tldr_newsletter::infrastructure::database::init_database::init_sqlite_database;
use tldr_newsletter::infrastructure::database::repository::feed_repository::FeedRepository;
use tldr_newsletter::infrastructure::email::gmail::GmailAdapter;
use tldr_newsletter::infrastructure::notification::discord::DiscordAdapter;
use tldr_newsletter::ports::ai_i::AiI;
use tldr_newsletter::ports::email::EmailI;

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

    let ai_client = match GeminiAdapter::new(&settings.ai) {
        Ok(a) => {
            info!("AI client is setup");
            a
        }
        Err(err) => {
            error!("Fail to init the AI adapter: {}", err);
            std::process::exit(1);
        }
    };

    let notification_client = match DiscordAdapter::new(&settings.notification.discord.webhook_url) {
        Ok(n) => {
            info!("Notification client is setup");
            n
        }
        Err(_) => {
            error!("Fail to init the notification client");
            std::process::exit(1);
        }
    };

    let database = match init_sqlite_database(&settings.database).await {
        Ok(database) => {
            info!("Database is initialized");
            database
        },
        Err(err) => {
            error!("Fail to init SqLite database {}", err);
            std::process::exit(1);
        }
    };

    let feed_repository : FeedRepository = FeedRepository::new(database);

    // Init all feed in database
    let feeds = &settings.rss.feeds;
    let mut entities : Vec<FeedEntity> = vec![];
    for feed in feeds {
        match  feed_repository.save(&Feed {
            title: feed.title.to_string(),
            url: feed.url.to_string(),
            feed_type: feed.feed_type.to_string(),
            is_active: feed.is_active,
        }).await {
            Ok(entity) => entities.push(entity),
            Err(err) => info!("Fail to save feed {}", err),
        }

    }


    rss_processor().await;

    // if settings.ai.enable {
    //     info!("Run Ai processor");
    //     match ai_processor(&settings.ai, &ai_client, &notification_client, &files).await {
    //         Ok(a) => a,
    //         Err(e) => {
    //             error!("Error during the AI processor {}", e);
    //             std::process::exit(1);
    //         }
    //     };
    // } else {
    //     info!("Ai processor disabled");
    // }
}
