use crate::infrastructure::rss::rss_client::RssAdapter;
use log::info;

pub async fn rss_processor() {
    let client = RssAdapter::new();

    let channel = client.fetch("https://bullrich.dev/tldr-rss/devops.rss")
        .await.expect("TODO: panic message");

    info!("{} items from the channel {}", channel.items.len(), channel.title);


}