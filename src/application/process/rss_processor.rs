use crate::domain::database::model::FeedEntity;
use crate::infrastructure::database::repository::feed_item_repository::FeedItemRepository;
use crate::infrastructure::rss::feed_item_mapper::map_channel_item;
use crate::infrastructure::rss::rss_client::RssAdapter;
use log::info;

#[derive(Clone, Debug)]
pub struct RssProcessor {
    client: RssAdapter,
    feed_item_repository: FeedItemRepository,
}

impl RssProcessor {
    pub fn new(rss_adapter: &RssAdapter, feed_item_repository: &FeedItemRepository, ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RssProcessor {
            client: rss_adapter.clone(),
            feed_item_repository: feed_item_repository.clone(),
        })
    }
    pub async fn process(
        &self,
        feeds: &Vec<FeedEntity>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for feed in feeds {
            self.process_channel(feed).await?;
        }
        Ok(())
    }

    /// Fetch channel content and save it
    async fn process_channel(&self, feed: &FeedEntity) -> Result<(), Box<dyn std::error::Error>> {
        let channel = self.client.fetch(&feed.url).await?;

        for item in &channel.items {
            let feed_item = map_channel_item(item);
            self.feed_item_repository.save(&feed_item, &feed.id).await?;
        }
        info!("Fetch and save {} items from the channel {}", channel.items.len(), channel.title);
        Ok(())
    }
}
