use async_trait::async_trait;
use std::time::Duration;
use serde::{Serialize, de::DeserializeOwned};

pub type CacheResult<T> = Result<T, CacheError>;
pub type CacheKey = String;
pub type Ttl = Option<Duration>;

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Redis error: {0}")]
    Redis(String),
    #[error("Invalid configuration: {0}")]
    Config(String),
}

pub mod redis_pool;
pub mod session_store;
pub mod token_blacklist;
pub mod rate_limiter;
pub mod basic_cache;

pub use basic_cache::BasicCache;

#[async_trait]
pub trait CacheManager: BasicCache {
    // Typed operations
    async fn get_typed<T: DeserializeOwned + Send + Sync>(&self, key: &str) -> CacheResult<Option<T>>;
    async fn set_typed<T: Serialize + Send + Sync>(&self, key: &str, value: &T, ttl: Option<Duration>) -> CacheResult<()>;
    
    // Extended operations
    async fn get_or_set<T, F>(&self, key: &str, ttl: Option<Duration>, f: F) -> CacheResult<T>
    where
        T: DeserializeOwned + Serialize + Send + Sync,
        F: FnOnce() -> T + Send + Sync;
        
    // Basic extended operations with defaults
    async fn increment(&self, key: &str, amount: i64) -> CacheResult<i64> {
        Err(CacheError::Config("Not implemented".to_string()))
    }
    
    async fn decrement(&self, key: &str, amount: i64) -> CacheResult<i64> {
        Err(CacheError::Config("Not implemented".to_string()))
    }
}

// Separate trait for advanced operations
#[async_trait]
pub trait AdvancedCacheManager: CacheManager {
    async fn get_multi<T: DeserializeOwned + Send + Sync>(&self, keys: &[String]) -> CacheResult<Vec<Option<T>>>;
    async fn set_multi<T: Serialize + Send + Sync>(&self, items: &[(&str, T)], ttl: Option<Duration>) -> CacheResult<()>;
    async fn hash_set(&self, key: &str, field: &str, value: &str) -> CacheResult<()>;
    async fn hash_get(&self, key: &str, field: &str) -> CacheResult<Option<String>>;
    async fn hash_get_all(&self, key: &str) -> CacheResult<std::collections::HashMap<String, String>>;
    async fn hash_delete(&self, key: &str, field: &str) -> CacheResult<()>;
    async fn list_push(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn list_pop(&self, key: &str) -> CacheResult<Option<String>>;
    async fn list_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>>;
    async fn set_add(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn set_remove(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn set_members(&self, key: &str) -> CacheResult<Vec<String>>;
    async fn set_is_member(&self, key: &str, value: &str) -> CacheResult<bool>;
    async fn sorted_set_add(&self, key: &str, score: f64, value: &str) -> CacheResult<()>;
    async fn sorted_set_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>>;
    async fn sorted_set_range_by_score(&self, key: &str, min: f64, max: f64) -> CacheResult<Vec<String>>;
    async fn publish(&self, channel: &str, message: &str) -> CacheResult<()>;
    async fn subscribe(&self, channels: &[String]) -> CacheResult<redis::aio::PubSub>;
    async fn flush_all(&self) -> CacheResult<()>;
    async fn ping(&self) -> CacheResult<String>;
}
