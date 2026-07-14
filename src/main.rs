use log::{error, info};
use tldr_newsletter::application::rss_processor::RssProcessor;
use tldr_newsletter::config::GlobalProperties;
use tldr_newsletter::domain::database::model::FeedEntity;
use tldr_newsletter::domain::rss::model::Feed;
use tldr_newsletter::infrastructure::ai::gemini::GeminiAdapter;
use tldr_newsletter::infrastructure::database::init_database::init_postgres_database;
use tldr_newsletter::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use tldr_newsletter::infrastructure::database::repository::feed_repository::FeedRepository;
use tldr_newsletter::infrastructure::notification::discord::DiscordAdapter;
use tldr_newsletter::infrastructure::rss::rss_client::RssAdapter;
use tldr_newsletter::ports::ai_i::AiI;

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

    let _ai_client = match GeminiAdapter::new(&settings.ai) {
        Ok(a) => {
            info!("AI client is setup");
            a
        }
        Err(err) => {
            error!("Fail to init the AI adapter: {}", err);
            std::process::exit(1);
        }
    };

    let _notification_client = match DiscordAdapter::new(&settings.notification.discord.webhook_url)
    {
        Ok(n) => {
            info!("Notification client is setup");
            n
        }
        Err(_) => {
            error!("Fail to init the notification client");
            std::process::exit(1);
        }
    };

    let database = match init_postgres_database(&settings.database).await {
        Ok(database) => {
            info!("Database is initialized");
            database
        }
        Err(err) => {
            error!("Fail to init Postgres database {}", err);
            std::process::exit(1);
        }
    };

    let feed_repository: FeedRepository = FeedRepository::new(&database);
    let feed_item_repository: FeedItemRepository = FeedItemRepository::new(&database);

    let rss_processor = match RssProcessor::new(&RssAdapter::new(), &feed_item_repository) {
        Ok(processor) => {
            info!("RSS processor is initialized");
            processor
        }
        Err(err) => {
            error!("Fail to init RSS adapter: {}", err);
            std::process::exit(1);
        }
    };

    // Init all feed in database
    let mut feeds: Vec<FeedEntity> = vec![];
    for feed in &settings.rss.feeds {
        match feed_repository
            .save(&Feed {
                title: feed.title.to_string(),
                url: feed.url.to_string(),
                feed_type: feed.feed_type,
                is_active: feed.is_active,
            })
            .await
        {
            Ok(entity) => feeds.push(entity),
            Err(err) => info!("Fail to save feed {}", err),
        }
    }

    if settings.rss.enable {
        info!("Run RSS processor");
        match rss_processor.process(&feeds).await {
            Ok(_) => info!("RSS processor finished"),
            Err(err) => {
                error!("Fail to process RSS: {}", err);
                std::process::exit(1);
            }
        }
    }

    if settings.ai.enable {
        info!("Run Ai processor");
        // match ai_processor(&settings.ai, &ai_client, &notification_client, &files).await {
        //     Ok(a) => a,
        //     Err(e) => {
        //         error!("Error during the AI processor {}", e);
        //         std::process::exit(1);
        //     }
        // };
    } else {
        info!("Ai processor disabled");
    }
}
