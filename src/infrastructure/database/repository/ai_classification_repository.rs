use crate::domain::ai::model::AiClassificationResponse;
use crate::domain::database::model::AiClassificationEntity;
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AiClassificationRepository {
    pool: PgPool,
}

impl AiClassificationRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn save(
        &self,
        item: &AiClassificationResponse,
    ) -> Result<AiClassificationEntity, sqlx::Error> {
        let saved = sqlx::query_as::<_, AiClassificationEntity>(
            r#"
                INSERT INTO AI_CLASSIFICATION(content)
                VALUES($1)
                RETURNING id::bigint,content::json, created_at, updated_at
            "#,
        )
        .bind(Json(item))
        .fetch_one(&self.pool)
        .await?;

        Ok(saved)
    }
}
