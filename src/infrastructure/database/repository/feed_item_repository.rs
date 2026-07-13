use crate::domain::database::model::FeedItemEntity;
use crate::domain::rss::model::FeedItem;
use sqlx::PgPool;

#[derive(Clone)]
pub struct FeedItemRepository {
    pool: PgPool,
}

impl FeedItemRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(&self, item: &FeedItem, feed_id: &i64) -> Result<FeedItemEntity, sqlx::Error> {
        if let Some(existing_feed_item) = self.find_by_guid(&item.guid).await? {
            return Ok(existing_feed_item);
        }

        let saved = sqlx::query_as::<_, FeedItemEntity>(
            r#"
                insert into FEED_ITEMS (feed_id, guid, title, url, content, published_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id::bigint, feed_id::bigint, guid, title, url, content, published_at, created_at, updated_at
                "#)
            .bind(feed_id)
            .bind(&item.guid)
            .bind(&item.title)
            .bind(&item.url)
            .bind(&item.content)
            .bind(item.published_at)
            .bind(chrono::Utc::now())
            .bind(chrono::Utc::now())
            .fetch_one(&self.pool)
            .await?;

        Ok(saved)
    }

    pub async fn find_by_guid(&self, guid: &str) -> Result<Option<FeedItemEntity>, sqlx::Error> {
        let feed_item =
            sqlx::query_as::<_, FeedItemEntity>("SELECT id::bigint, feed_id::bigint, guid, url, title, content, published_at, created_at, updated_at FROM FEED_ITEMS WHERE guid = $1;")
                .bind(guid)
                .fetch_optional(&self.pool)
                .await?;

        Ok(feed_item)
    }
}
