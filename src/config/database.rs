use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub pool: MySqlPool,
}

impl DatabaseConfig {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
