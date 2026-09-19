use crate::domain::ai::model::AiClassificationResponse;
use crate::domain::database::model::AiClassificationEntity;
use chrono::NaiveDate;
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Clone, Debug)]
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
        feed_items: &Vec<i64>,
    ) -> Result<AiClassificationEntity, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let classification = sqlx::query_as::<_, AiClassificationEntity>(
            r#"
                INSERT INTO AI_CLASSIFICATION(content)
                VALUES($1)
                RETURNING id::bigint,content::json, created_at, updated_at
            "#,
        )
        .bind(Json(item))
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
                    INSERT INTO ai_classification_feed_items (ai_classification_id, feed_item_id)
                    SELECT $1, unnest($2::bigint[])
                "#,
        )
        .bind(classification.id)
        .bind(&feed_items.iter().map(|id| id).collect::<Vec<_>>())
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(classification)
    }
    pub async fn get_by_creation_date(
        &self,
        date: &NaiveDate,
    ) -> Result<Option<AiClassificationEntity>, sqlx::Error> {
        let entity: Option<AiClassificationEntity> = sqlx::query_as::<_, AiClassificationEntity>(r#"
            SELECT id::bigint,content::json, created_at, updated_at FROM AI_CLASSIFICATION ac WHERE DATE(ac.created_at) = $1
        "#)
            .bind(date)
            .fetch_optional(&self.pool)
            .await?;

        Ok(entity)
    }
}
