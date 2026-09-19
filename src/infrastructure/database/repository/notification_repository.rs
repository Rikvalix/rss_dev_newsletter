use crate::domain::database::model::NotificationEntity;
use sqlx::PgPool;

#[derive(Clone,Debug)]
pub struct NotificationRepository {
    pool: PgPool,
}

impl NotificationRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn get_all(&self, active: bool) -> Result<Vec<NotificationEntity>, sqlx::Error> {
        let all = sqlx::query_as::<_, NotificationEntity>("
                SELECT id::bigint,target,url,target_user,active,created_at,updated_at FROM Notification
                WHERE active = $1")
            .bind(active)
            .fetch_all(&self.pool)
            .await?;

        Ok(all)
    }
}
