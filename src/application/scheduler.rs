use crate::domain::application::model::ApplicationConfiguration;
use std::time::Duration;
use tokio::time;

pub async fn init_scheduled_task(application_configuration: &ApplicationConfiguration) {


    // RSS processor
    let rss_processor = application_configuration.rss_processor.clone();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            rss_processor.process()
                .await
                .unwrap();
        }
    });

    // AI processor
    let ai_processor = application_configuration.ai_processor;
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            ai_processor.process()
                .await
                .unwrap()
        }
    })
}