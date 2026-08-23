pub mod member_repository;
pub mod text_repository;
pub mod donation_repository;
pub mod plagiarism_repository;
pub mod committee_repository;
pub mod charity_repository;
pub mod notification_repository;

// Re-export for convenience
pub use member_repository::*;
pub use text_repository::*;
pub use donation_repository::*;
pub use plagiarism_repository::*;
pub use committee_repository::*;
pub use charity_repository::*;
pub use notification_repository::*;

use uuid::Uuid;
use thiserror::Error;
use async_trait::async_trait;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),
    
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<T>;
    async fn save(&self, entity: &T) -> RepositoryResult<()>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<()>;
    async fn exists(&self, id: Uuid) -> RepositoryResult<bool>;
}

#[async_trait]
pub trait PaginatedRepository<T>: Repository<T> {
    async fn find_all(&self, page: u32, limit: u32) -> RepositoryResult<Vec<T>>;
    async fn count(&self) -> RepositoryResult<u64>;
}

// Common repository traits for specific operations
#[async_trait]
pub trait SearchableRepository<T>: Send + Sync {
    async fn search(
        &self,
        query: Option<&str>,
        filters: &[(&str, String)],
        page: u32,
        limit: u32,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> RepositoryResult<(Vec<T>, u64)>;
}

#[async_trait]
pub trait SoftDeleteRepository<T>: Repository<T> {
    async fn soft_delete(&self, id: Uuid) -> RepositoryResult<()>;
    async fn restore(&self, id: Uuid) -> RepositoryResult<()>;
    async fn find_deleted(&self) -> RepositoryResult<Vec<T>>;
}

// Common structs used across repositories
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PaginationResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub filters: Vec<(String, String)>,
    pub page: u32,
    pub limit: u32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            query: None,
            filters: Vec::new(),
            page: 1,
            limit: 20,
            sort_by: None,
            sort_order: None,
        }
    }
}

// Helper functions for repository implementations
pub mod helpers {
    use sqlx::{Postgres, Transaction};
    use super::RepositoryError;

    pub async fn execute_in_transaction<T, F>(
        transaction: &mut Transaction<'_, Postgres>,
        operation: F,
    ) -> Result<T, RepositoryError>
    where
        F: FnOnce(&mut Transaction<'_, Postgres>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, RepositoryError>> + Send>>,
    {
        match operation(transaction).await {
            Ok(result) => {
                transaction.commit().await
                    .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;
                Ok(result)
            }
            Err(e) => {
                transaction.rollback().await
                    .map_err(|e| RepositoryError::TransactionError(e.to_string()))?;
                Err(e)
            }
        }
    }

    pub fn map_sqlx_error(e: sqlx::Error, entity_name: &str) -> RepositoryError {
        match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(format!("{} not found", entity_name)),
            sqlx::Error::Database(db_err) => {
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "23505" => RepositoryError::DuplicateEntry(format!("Duplicate {} entry", entity_name)),
                        "23503" => RepositoryError::ConstraintViolation(format!("Foreign key constraint violation for {}", entity_name)),
                        "23502" => RepositoryError::ValidationError(format!("Not null constraint violation for {}", entity_name)),
                        "23514" => RepositoryError::ValidationError(format!("Check constraint violation for {}", entity_name)),
                        _ => RepositoryError::DatabaseError(sqlx::Error::Database(db_err)),
                    }
                } else {
                    RepositoryError::DatabaseError(sqlx::Error::Database(db_err))
                }
            }
            _ => RepositoryError::DatabaseError(e),
        }
    }
}
