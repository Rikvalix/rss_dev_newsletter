use crate::infrastructure::rss::rss_client::RssAdapter;

pub async fn rss_processor() {

    let client = RssAdapter::new();

    client.fetch("https://bullrich.dev/tldr-rss/devops.rss")
        .await.expect("TODO: panic message");
}