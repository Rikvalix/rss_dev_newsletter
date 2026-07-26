use log::{error, info};
use rss_dev_newsletter::application::process::ai_processor::AiProcessor;
use rss_dev_newsletter::application::process::rss_processor::RssProcessor;
use rss_dev_newsletter::application::web_server::run_web_server;
use rss_dev_newsletter::config::GlobalProperties;
use rss_dev_newsletter::domain::application::model::ApplicationConfiguration;
use rss_dev_newsletter::domain::database::model::RepositoryRepoHandler;
use rss_dev_newsletter::infrastructure::ai::mistral::MistralAdapter;
use rss_dev_newsletter::infrastructure::database::init_database::init_postgres_database;
use rss_dev_newsletter::infrastructure::database::repository::ai_classification_repository::AiClassificationRepository;
use rss_dev_newsletter::infrastructure::database::repository::ai_summary_repository::AiSummaryRepository;
use rss_dev_newsletter::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use rss_dev_newsletter::infrastructure::database::repository::feed_repository::FeedRepository;
use rss_dev_newsletter::infrastructure::database::repository::notification_repository::NotificationRepository;
use rss_dev_newsletter::infrastructure::database::repository::summary_repository::SummaryRepository;
use rss_dev_newsletter::infrastructure::rss::rss_client::RssAdapter;
use rss_dev_newsletter::ports::ai_i::AiI;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing_subscriber::fmt::init();

    let settings = match GlobalProperties::new() {
        Ok(settings) => settings,
        Err(err) => {
            error!("Fail to init configuration {}", err);
            std::process::exit(1);
        }
    };
    info!("Configuration is setup");

    let database = init_postgres_database(&settings.database)
        .await
        .unwrap_or_else(|err| {
            error!("Fail to init Postgres database {}", err);
            std::process::exit(1);
        });

    info!("Database is initialized");

    let application_configuration = init_application_configuration(&settings, &database);

    info!("Application configuration is initialized");

    run_web_server(&application_configuration).await;
}

fn init_application_configuration(
    settings: &GlobalProperties,
    database: &PgPool,
) -> ApplicationConfiguration {
    let repository_handler = RepositoryRepoHandler {
        ai_classification_repository: AiClassificationRepository::new(&database),
        ai_summary_repository: AiSummaryRepository::new(&database),
        feed_item_repository: FeedItemRepository::new(&database),
        feed_repository: FeedRepository::new(&database),
        notification_repository: NotificationRepository::new(&database),
        summary_repository: SummaryRepository::new(&database),
    };

    let ai_client = MistralAdapter::new(&settings.ai).unwrap_or_else(|err| {
        error!("Fail to init RSS processor: {err}");
        std::process::exit(1);
    });

    let rss_processor =
        RssProcessor::new(&RssAdapter::new(), &repository_handler.feed_item_repository)
            .unwrap_or_else(|err| {
                error!("Fail to init RSS processor: {err}");
                std::process::exit(1);
            });

    let ai_processor = AiProcessor::new(
        ai_client,
        &repository_handler.ai_classification_repository,
        &repository_handler.ai_summary_repository,
    )
    .unwrap_or_else(|err| {
        error!("Fail to init AI processor: {err}");
        std::process::exit(1);
    });

    ApplicationConfiguration {
        repositories: repository_handler,
        rss_processor,
        ai_processor,
    }
}
