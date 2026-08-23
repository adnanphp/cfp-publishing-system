// backend/src/infrastructure/mod.rs
pub mod database;
pub mod cache;
pub mod security;
pub mod messaging;
pub mod external;

// Re-exports
pub use database::DatabasePool;
pub use cache::redis_pool::RedisPool;
pub use security::*;
