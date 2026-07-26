use chrono::Utc;
use log::{error, info};
use rss_dev_newsletter::application::ai_processor::ai_processor;
use rss_dev_newsletter::application::rss_processor::RssProcessor;
use rss_dev_newsletter::config::GlobalProperties;
use rss_dev_newsletter::domain::database::model::SummaryMetadata;
use rss_dev_newsletter::infrastructure::ai::mistral::MistralAdapter;
use rss_dev_newsletter::infrastructure::database::init_database::init_postgres_database;
use rss_dev_newsletter::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use rss_dev_newsletter::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use rss_dev_newsletter::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use rss_dev_newsletter::infrastructure::database::repository::feed_repository::FeedRepository;
use rss_dev_newsletter::infrastructure::database::repository::notification_repository::NotificationRepository;
use rss_dev_newsletter::infrastructure::database::repository::summary_repository::SummaryRepository;
use rss_dev_newsletter::infrastructure::markdown_generator::generator::summary_generator;
use rss_dev_newsletter::infrastructure::notification::discord::DiscordAdapter;
use rss_dev_newsletter::infrastructure::rss::rss_client::RssAdapter;
use rss_dev_newsletter::ports::ai_i::AiI;
use rss_dev_newsletter::ports::notification_i::NotificationI;

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

    let ai_client = match MistralAdapter::new(&settings.ai) {
        Ok(a) => {
            info!("AI client is setup");
            a
        }
        Err(err) => {
            error!("Fail to init the AI adapter: {}", err);
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
    let ai_classification_repository: AiClassificationRepository =
        AiClassificationRepository::new(&database);
    let ai_summary_repository: AiSummaryRepository = AiSummaryRepository::new(&database);
    let summary_repository: SummaryRepository = SummaryRepository::new(&database);
    let notification_repository: NotificationRepository = NotificationRepository::new(&database);

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

    if settings.rss.enable {
        info!("Run RSS processor");

        let feeds = feed_repository
            .find_all(true)
            .await
            .map_err(|err| error!("Fail to find feeds: {}", err))
            .unwrap();

        match rss_processor.process(&feeds).await {
            Ok(_) => info!("RSS processor finished"),
            Err(err) => {
                error!("Fail to process RSS: {}", err);
                std::process::exit(1);
            }
        }
    }

    let today_feeds = feed_item_repository
        .find_by_creation_date(&Utc::now().date_naive())
        .await
        .map_err(|err| error!("Fail to find feeds items: {}", err))
        .unwrap();

    if settings.ai.enable {
        info!("Run Ai processor");
        let ai_summary = match ai_processor(
            &ai_client,
            &ai_classification_repository,
            &ai_summary_repository,
            &today_feeds,
        )
        .await
        {
            Ok(a) => a,
            Err(e) => {
                error!("Error during the AI processor {}", e);
                std::process::exit(1);
            }
        };
        //let ai_summary = ai_summary_repository.find_latest().await.unwrap();


        let markdown = summary_generator(&ai_summary.content, &today_feeds);

        let metadata = SummaryMetadata {
            model: Some(settings.ai.mistral.model),
        };

        summary_repository
            .save(&ai_summary, &markdown, &metadata)
            .await
            .unwrap();

        // Send notifications
        let targets = notification_repository.get_all(true).await.unwrap();
        for target in targets {
            let client = match DiscordAdapter::new(&target.url.as_str()) {
                Ok(client) => client,
                Err(error) => {
                    error!("Error when create discord client {}", error);
                    std::process::exit(1);
                }
            };


            client.send_summary_file(&Utc::now().naive_utc().date(),&target.target_user,&markdown).await.unwrap()
        }
    } else {
        info!("Ai processor disabled");
    }
}
