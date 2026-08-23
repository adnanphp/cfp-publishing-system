use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

// Simple repository implementations
pub mod repositories {
    pub mod member_repository;
    pub mod author_repository;
    pub mod text_repository;
    pub mod donation_repository;  // Add this line
    
    pub use member_repository::MemberRepository;
    //pub use author_repository::AuthorRepository;
    //pub use text_repository::TextRepository;
      // Add this line
}

// Re-export for convenience
