use crate::domain::database::model::FeedEntity;
use crate::domain::rss::model::Feed;
use sqlx::PgPool;

#[derive(Clone)]
pub struct FeedRepository {
    pool: PgPool,
}

impl FeedRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(&self, feed: &Feed) -> Result<FeedEntity, sqlx::Error> {
        if let Some(existing_feed) = self.find_by_title(&feed.title).await? {
            return Ok(existing_feed);
        }

        let saved = sqlx::query_as::<_, FeedEntity>(
            r#"
            insert into FEEDS (title, url, feed_type, is_active)
            VALUES($1,$2,$3,$4)
            RETURNING id::bigint, title, url, feed_type, is_active, updated_at,created_at
            "#,
        )
        .bind(&feed.title)
        .bind(&feed.url)
        .bind(&feed.feed_type.to_string())
        .bind(feed.is_active)
        .fetch_one(&self.pool)
        .await?;

        Ok(saved)
    }

    pub async fn find_all(&self, is_active: bool) -> Result<Vec<FeedEntity>, sqlx::Error> {
        let feeds = sqlx::query_as::<_, FeedEntity>("SELECT id::bigint, title, url, feed_type, is_active,created_at, updated_at FROM FEEDS f WHERE f.is_active = $1")
            .bind(is_active)
        .fetch_all(&self.pool)
            .await?;

        Ok(feeds)
    }
    pub async fn find_by_title(&self, title: &str) -> Result<Option<FeedEntity>, sqlx::Error> {
        let feed = sqlx::query_as::<_, FeedEntity>("select id::bigint, title, url, feed_type, is_active, created_at, updated_at from feeds where title = $1")
            .bind(title)
            .fetch_optional(&self.pool)
            .await?;

        Ok(feed)
    }
}
