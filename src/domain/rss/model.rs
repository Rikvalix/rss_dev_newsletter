#[derive(Debug, Default)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub feed_type: String,
    pub is_active: bool,
}

#[derive(Debug, Default)]
pub struct FeedItem {
    pub feed_id: Option<u32>,
    pub guid: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
}
