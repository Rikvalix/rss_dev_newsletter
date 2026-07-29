use crate::application::process::ai_processor::AiProcessor;
use crate::application::process::markdown_notification_process::MarkdownNotificationProcessor;
use crate::application::process::rss_processor::RssProcessor;
use crate::domain::application::model::ApplicationConfiguration;
use crate::infrastructure::ai::mistral::MistralAdapter;
use crate::infrastructure::rss::rss_client::RssAdapter;
use crate::ports::ai_i::AiI;
use log::{error, info};
use std::sync::Arc;
use tokio_task_scheduler::{Scheduler, TaskBuilder};

pub async fn init_scheduled_task(application_configuration: &ApplicationConfiguration) {
    let scheduler = Scheduler::new();

    let repository_handler = &application_configuration.repositories;

    let rss_processor = Arc::new(
        RssProcessor::new(
            &RssAdapter::new(),
            &repository_handler.feed_item_repository,
            &repository_handler.feed_repository,
        )
        .unwrap_or_else(|err| {
            error!("Fail to init RSS processor: {err}");
            std::process::exit(1);
        }),
    );

    let ai_client =
        MistralAdapter::new(&application_configuration.config.ai).unwrap_or_else(|err| {
            error!("Fail to init RSS processor: {err}");
            std::process::exit(1);
        });

    let ai_processor = Arc::new(
        AiProcessor::new(
            ai_client,
            &repository_handler.ai_classification_repository,
            &repository_handler.ai_summary_repository,
            &repository_handler.feed_item_repository,
        )
        .unwrap_or_else(|err| {
            error!("Fail to init AI processor: {err}");
            std::process::exit(1);
        }),
    );

    let markdown_notification_processor = Arc::new(
        MarkdownNotificationProcessor::new(
            &repository_handler.ai_summary_repository,
            &repository_handler.feed_item_repository,
            &repository_handler.notification_repository,
            &repository_handler.summary_repository,
        )
        .unwrap_or_else(|err| {
            error!("Fail to init Markdown / Notification processor: {err}");
            std::process::exit(1);
        }),
    );

    // RSS processor
    if application_configuration.config.rss.enable {
        let rss_process_task = TaskBuilder::new("rss_processor", move || {
            let rss_processor_clone = Arc::clone(&rss_processor);

            tokio::spawn(async move {
                info!("Starting scheduled RSS processing");
                if let Err(err) = rss_processor_clone.process().await {
                    error!("Error during RSS processing: {}", err);
                }
            });
            Ok(())
        })
        .every_seconds(30)
        .build();

        let rss_processor_task = scheduler
            .add_task(rss_process_task)
            .await
            .unwrap_or_else(|err| {
                error!("Failed to initialize RSS processor task: {err}");
                std::process::exit(1);
            });
        info!("Registered RSS processor task: {rss_processor_task}");
    }

    // AI processor

    if application_configuration.config.ai.enable {
        let ai_process_task = TaskBuilder::new("ai_processor", move || {
            let ai_processor_clone = Arc::clone(&ai_processor);
            
            tokio::spawn(async move {
                info!("Starting scheduled AI processing");
                    if let Err(err) = ai_processor_clone.process().await {
                        error!("Error during AI processing: {:?}", err);
                    }
            });
            Ok(())
        })
        .daily()
        .at("20:04")
        .unwrap()
        .build();

        let ai_processor_task = scheduler
            .add_task(ai_process_task)
            .await
            .unwrap_or_else(|err| {
                error!("Failed to initialize AI processor task: {err}");
                std::process::exit(1);
            });
        info!("Registered AI processor task: {ai_processor_task}");
    }

    if application_configuration.config.notification.enable {
        let markdown_notification_task =
            TaskBuilder::new("markdown_notification_processor", move || {
                let markdown_notification_clone = Arc::clone(&markdown_notification_processor);

                tokio::spawn(async move {
                    info!("Starting notification processor task");
                    if let Err(err) = markdown_notification_clone.process().await {
                        error!("Error during notification processor task: {err}");
                    }
                });
                Ok(())
            })
            .daily()
            .at("22:01")
            .unwrap()
            .build();

        let markdown_notification_task_registration = scheduler
            .add_task(markdown_notification_task)
            .await
            .unwrap_or_else(|err| {
                error!("Failed to initialize Markdown / Notification processor task: {err}");
                std::process::exit(1);
            });
        info!(
            "Registered Markdown / Notification processor task: {markdown_notification_task_registration}"
        );
    }

    scheduler.start().await;
}
