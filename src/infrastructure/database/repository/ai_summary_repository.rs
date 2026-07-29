use crate::domain::ai::model::AiSummaryResponse;
use crate::domain::database::model::{AiSummaryEntity, AiSummaryWithFeedItems, FeedItemEntity};
use chrono::NaiveDate;
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AiSummaryRepository {
    pool: PgPool,
}

impl AiSummaryRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(
        &self,
        summary: &AiSummaryResponse,
        ai_classification_id: &i64,
    ) -> Result<AiSummaryEntity, sqlx::Error> {
        let saved = sqlx::query_as::<_, AiSummaryEntity>(
            r#"
                    INSERT INTO ai_summary(content,ai_classification_id)
                    VALUES ($1, $2)
                     RETURNING id::bigint,ai_classification_id::bigint, content::json, created_at, updated_at
                "#,
        )
        .bind(Json(summary))
        .bind(ai_classification_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(saved)
    }

    pub async fn find_latest(&self) -> Result<AiSummaryEntity, sqlx::Error> {
        let response = sqlx::query_as::<_, AiSummaryEntity>(
            r#"
                SELECT id::bigint,ai_classification_id::bigint, content::json, created_at, updated_at FROM AI_SUMMARY
                  ORDER BY ID::bigint DESC
                  LIMIT 1
                "#)
            .fetch_one(&self.pool)
            .await?;

        Ok(response)
    }

    pub async fn get_by_creation_date_with_feed_items(
        &self,
        date: &NaiveDate,
    ) -> Result<Option<AiSummaryWithFeedItems>, sqlx::Error> {
        let ai_summary = sqlx::query_as::<_, AiSummaryEntity>(r#"
            SELECT id::bigint,ai_classification_id::bigint, content::json, created_at, updated_at FROM AI_SUMMARY ac WHERE DATE(ac.created_at) = $1
        "#)
            .bind(date)
            .fetch_optional(&self.pool)
            .await?;

        if ai_summary.is_none() {
            return Ok(None);
        }
        let ai_summary = ai_summary.unwrap();

        let feed_items: Vec<FeedItemEntity> = sqlx::query_as::<_, FeedItemEntity>(
            r#"
            SELECT FROM feed_items  fi
                inner join ai_classification_feed_items acfi on acfi.feed_item_id = fi.id
                where acfi.ai_classification_id = $1
            "#,
        )
        .bind(ai_summary.id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(AiSummaryWithFeedItems {
            ai_summary,
            feed_items,
        }))
    }
}
