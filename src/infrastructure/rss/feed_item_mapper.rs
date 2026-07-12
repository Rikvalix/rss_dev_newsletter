use crate::domain::rss::model::FeedItem;
use regex::Regex;
use rss::Item;
use uuid::Uuid;


/// Map current channel item to FeedItem model
/// Auto detect URL in guid to move it in to link field
///
/// # Arguments
///
/// * `item`: Item from the RSS channel
///
/// returns: FeedItem
pub fn map_channel_item(item: &Item) -> FeedItem {

    let re_url = Regex::new(r"(https:\/\/www\.|http:\/\/www\.|https:\/\/|http:\/\/)?[a-zA-Z0-9]{2,}(\.[a-zA-Z0-9]{2,})(\.[a-zA-Z0-9]{2,})?").unwrap();

    let mut guid: String = match &item.guid {
        Some(guid) => guid.value().to_string(),
        None => Uuid::new_v4().to_string(),
    };

    let mut link = String::new();
    if re_url.is_match(&guid) {
        link.push_str(&guid);
        guid = Uuid::new_v4().to_string();
    }

    FeedItem {
        guid,
        url: item.link.clone().unwrap_or_else(|| link.clone()),
        title: item.title.clone().unwrap_or(String::new()),
        description: item.description.clone().unwrap_or(String::new()),
        content: item.content.clone().unwrap_or(String::new()),
        
        ..Default::default()
    }
}


