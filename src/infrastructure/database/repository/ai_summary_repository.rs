use crate::domain::ai::model::AiSummaryResponse;
use crate::domain::database::model::AiSummaryEntity;
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Clone,Debug)]
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
}
