pub mod database_pool;
pub mod repositories;
pub mod migrations;
pub mod queries;

// Re-export for convenience
pub use database_pool::*;
pub use repositories::*;
pub use migrations::*;
pub use queries::*;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use crate::config::database::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .max_lifetime(Duration::from_secs(60 * 60)) // 1 hour
            .idle_timeout(Duration::from_secs(10 * 60)) // 10 minutes
            .acquire_timeout(Duration::from_secs(30))
            .connect(&config.database_url)
            .await?;

        // Test the connection
        sqlx::query("SELECT 1").execute(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("Database migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    
    #[error("Repository error: {0}")]
    RepositoryError(#[from] crate::infrastructure::database::repositories::RepositoryError),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;
