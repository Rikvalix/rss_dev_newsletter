use crate::config::DatabaseProperties;
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_postgres_database(
    properties: &DatabaseProperties,
) -> Result<PgPool, Box<dyn std::error::Error>> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        properties.user, properties.password, properties.host, properties.port, properties.database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;

    info!("Database connected");

    sqlx::migrate!("src/infrastructure/database/migrations")
        .run(&pool)
        .await?;

    info!("Migrations are applied successfully");

    Ok(pool)
}
