use async_trait::async_trait;
use std::time::Duration;

use super::{CacheResult, CacheError};

#[async_trait]
pub trait BasicCache: Send + Sync {
    async fn get(&self, key: &str) -> CacheResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()>;
    async fn delete(&self, key: &str) -> CacheResult<()>;
    async fn exists(&self, key: &str) -> CacheResult<bool>;
    async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<()>;
}
