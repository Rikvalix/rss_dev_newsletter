use crate::domain::database::model::{AiSummaryEntity, SummaryEntity, SummaryMetadata};
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Debug,Clone)]
pub struct SummaryRepository {
    pub pool: PgPool,
}

impl SummaryRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(
        &self,
        ai_summary_response: &AiSummaryEntity,
        markdown_content: &String,
        metadata: &SummaryMetadata,
    ) -> Result<SummaryEntity, sqlx::Error> {
        let saved = sqlx::query_as::<_, SummaryEntity>(r#"
                INSERT INTO SUMMARY (ai_summary_id,title,content,metadata)
                VALUES ($1,$2,$3,$4)
                RETURNING id::bigint, public_id::uuid,ai_summary_id::bigint,title,content,metadata, created_at, updated_at"#)
            .bind(&ai_summary_response.id)
            .bind(&ai_summary_response.content.global_title)
            .bind(markdown_content)
            .bind(Json(metadata))
            .fetch_one(&self.pool)
            .await?;

        Ok(saved)
    }

    pub async fn find_latest(&self) -> Result<SummaryEntity, sqlx::Error> {
        let saved =
            sqlx::query_as::<_, SummaryEntity>("SELECT id::bigint,public_id::uuid,ai_summary_id::bigint,title,content,metadata,created_at,updated_at FROM SUMMARY ORDER BY ID::bigint DESC LIMIT 1")
                .fetch_one(&self.pool)
                .await?;
        Ok(saved)
    }
}
