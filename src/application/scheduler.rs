use crate::application::process::ai_processor::AiProcessor;
use crate::application::process::markdown_notification_process::MarkdownNotificationProcessor;
use crate::application::process::rss_processor::RssProcessor;
use crate::domain::application::model::ApplicationConfiguration;
use crate::infrastructure::ai::mistral::MistralAdapter;
use crate::infrastructure::rss::rss_client::RssAdapter;
use crate::ports::ai_i::AiI;
use log::{error, info, warn};
use std::sync::Arc;
use tokio_cron_scheduler::{JobBuilder, JobScheduler};

pub async fn init_scheduled_task(application_configuration: &ApplicationConfiguration) {
    let scheduler = JobScheduler::new().await.unwrap();

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
            &application_configuration.config.notification.website_newsletter
        )
        .unwrap_or_else(|err| {
            error!("Fail to init Markdown / Notification processor: {err}");
            std::process::exit(1);
        }),
    );

    // RSS processor
    if application_configuration.config.rss.enable {
        let rss_processor_job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Paris)
            .with_cron_job_type()
            .with_schedule(&application_configuration.config.rss.cron_string)
            .unwrap()
            .with_run_async(Box::new(move |uuid, mut locked| {
                let rss_processor_job = Arc::clone(&rss_processor);
                Box::pin(async move {
                    info!("Starting scheduled RSS processing: {:?}", uuid);
                    let next_tick = locked.next_tick_for_job(uuid).await;
                    if let Err(err) = rss_processor_job.process().await {
                        error!("Error during RSS processing: {}", err);
                    }
                    match next_tick {
                        Ok(Some(ts)) => info!("Next time for RSS processor job is {:?}", ts),
                        _ => warn!("Could not get next tick for RSS processor job"),
                    }
                })
            }))
            .build()
            .unwrap();

        let _ = scheduler.add(rss_processor_job).await;
    }

    // AI processor

    if application_configuration.config.ai.enable {
        let ai_processor_job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Paris)
            .with_cron_job_type()
            .with_schedule(&application_configuration.config.ai.cron_string)
            .unwrap()
            .with_run_async(Box::new(move |uuid, mut locked| {
                let ai_proc_job = Arc::clone(&ai_processor);
                Box::pin(async move {
                    info!("Starting scheduled AI processing: {:?}", uuid);
                    let next_tick = locked.next_tick_for_job(uuid).await;
                    if let Err(err) = ai_proc_job.process().await {
                        error!("Error during AI processing: {}", err);
                    }
                    match next_tick {
                        Ok(Some(ts)) => info!("Next time for AI processor job is {:?}", ts),
                        _ => warn!("Could not get next tick for AI processor job"),
                    }
                })
            }))
            .build()
            .unwrap();

        let _ = scheduler.add(ai_processor_job).await;
    }

    if application_configuration.config.notification.enable {
        let markdown_notification_processor_job = JobBuilder::new()
            .with_timezone(chrono_tz::Europe::Paris)
            .with_cron_job_type()
            .with_schedule(&application_configuration.config.notification.cron_string)
            .unwrap()
            .with_run_async(Box::new(move |uuid, mut locked| {
                let markdown_notification_proc_job = Arc::clone(&markdown_notification_processor);
                Box::pin(async move {
                    info!(
                        "Starting scheduled Markdown / Notification processing: {:?}",
                        uuid
                    );
                    let next_tick = locked.next_tick_for_job(uuid).await;
                    if let Err(err) = markdown_notification_proc_job.process().await {
                        error!("Error during Markdown / Notification processing: {}", err);
                    }
                    match next_tick {
                        Ok(Some(ts)) => info!(
                            "Next time for Markdown / Notification processor job is {:?}",
                            ts
                        ),
                        _ => warn!(
                            "Could not get next tick for Markdown / Notification processor job"
                        ),
                    }
                })
            }))
            .build()
            .unwrap();

        let _ = scheduler.add(markdown_notification_processor_job).await;
    }


    scheduler.start().await.unwrap();
}
