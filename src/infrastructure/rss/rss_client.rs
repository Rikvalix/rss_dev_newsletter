use std::error::Error;

pub struct RssAdapter {
    client: reqwest::Client,
}

impl RssAdapter {
    pub fn new() -> Self {
        RssAdapter {
            client: reqwest::Client::new(),
        }
    }
    pub async fn fetch(&self, url: &str) -> Result<rss::Channel, Box<dyn Error>> {
        let rqt = self.client.get(url).send().await?.bytes().await?;
        let channel = rss::Channel::read_from(&rqt[..])?;
        Ok(channel)
    }
}
