use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{info, error};

pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("Connecting to database...");
        
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;
            
        info!("Database connection established");
        
        // Test connection
        sqlx::query("SELECT 1").execute(&pool).await?;
        info!("Database connection test successful");
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
}
