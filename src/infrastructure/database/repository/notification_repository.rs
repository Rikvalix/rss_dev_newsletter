use crate::domain::database::model::NotificationEntity;
use sqlx::PgPool;

pub struct NotificationRepository {
    pool: PgPool,
}

impl NotificationRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn get_all(&self) -> Result<Vec<NotificationEntity>, sqlx::Error> {
        let all = sqlx::query_as::<_, NotificationEntity>("
                SELECT id::bigint,target,url,target_user,active,created_at,updated_at FROM Notification")
            .fetch_all(&self.pool)
            .await?;

        Ok(all)
    }
}
