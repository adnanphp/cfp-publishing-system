pub mod member_queries;
pub mod text_queries;
pub mod donation_queries;
pub mod plagiarism_queries;

// Re-export for convenience
pub use member_queries::*;
pub use text_queries::*;
pub use donation_queries::*;
pub use plagiarism_queries::*;

use crate::utils::error::AppError;

#[async_trait]
pub trait QueryHandler<TQuery, TResult> {
    async fn handle(&self, query: TQuery) -> Result<TResult, AppError>;
}

pub trait Query: Send + Sync {
    fn query_name(&self) -> &'static str;
}
use async_trait::async_trait;
