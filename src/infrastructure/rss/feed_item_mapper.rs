use crate::domain::rss::model::FeedItem;
use regex::Regex;
use rss::Item;
use sha2::{Digest, Sha256};

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
        None => hash_guid(format!("{}{}",&item.title.clone().unwrap_or_default(),&item.link.clone().unwrap_or_default()).as_str()),
    };

    let mut link = String::new();
    if re_url.is_match(&guid) {
        link.push_str(&guid);
        guid =hash_guid(format!("{}{}",&item.title.clone().unwrap_or_default(),&item.link.clone().unwrap_or_default()).as_str());
    }

    FeedItem {
        guid,
        url: item.link.clone().unwrap_or_else(|| link.clone()),
        title: item.title.clone().unwrap_or_default(),
        description: item.description.clone().unwrap_or_default(),
        content: item.content.clone().unwrap_or_default(),
        ..Default::default()
    }
}

fn hash_guid(content: &str) -> String {
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher,content.as_bytes());
    hex::encode(hasher.finalize())
}
