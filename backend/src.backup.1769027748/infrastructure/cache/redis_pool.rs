use redis::{aio::ConnectionManager, RedisResult};
use std::time::Duration;
use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use tracing::{info, warn, error};

use super::{CacheManager, AdvancedCacheManager, CacheResult, CacheError, CacheKey, Ttl};

#[derive(Clone)]
pub struct RedisPool {
    connection_manager: ConnectionManager,
}

impl RedisPool {
    pub async fn new(url: &str) -> Result<Self, CacheError> {
        let client = redis::Client::open(url)
            .map_err(|e| CacheError::Connection(e.to_string()))?;
        
        let connection_manager = ConnectionManager::new(client)
            .await
            .map_err(|e| CacheError::Connection(e.to_string()))?;
        
        info!("Redis connection pool created successfully");
        Ok(RedisPool { connection_manager })
    }
    
    async fn get_connection(&self) -> Result<ConnectionManager, CacheError> {
        Ok(self.connection_manager.clone())
    }
}

#[async_trait]
impl CacheManager for RedisPool {
    async fn get(&self, key: &str) -> CacheResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let result: Option<String> = conn.get(key).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        
        match ttl {
            Some(ttl) => {
                let seconds = ttl.as_secs() as usize;
                conn.set_ex(key, value, seconds).await
                    .map_err(|e| CacheError::Redis(e.to_string()))?;
            }
            None => {
                conn.set(key, value).await
                    .map_err(|e| CacheError::Redis(e.to_string()))?;
            }
        }
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.del(key).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> CacheResult<bool> {
        let mut conn = self.get_connection().await?;
        let result: bool = conn.exists(key).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        let seconds = ttl.as_secs() as i64;
        conn.expire(key, seconds).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn get_typed<T: DeserializeOwned + Send + Sync>(&self, key: &str) -> CacheResult<Option<T>> {
        let value: Option<String> = self.get(key).await?;
        
        match value {
            Some(v) => {
                let parsed: T = serde_json::from_str(&v)
                    .map_err(|e| CacheError::Deserialization(e.to_string()))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }
    
    async fn set_typed<T: Serialize + Send + Sync>(&self, key: &str, value: &T, ttl: Option<Duration>) -> CacheResult<()> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| CacheError::Serialization(e.to_string()))?;
        self.set(key, &serialized, ttl).await
    }
    
    async fn get_or_set<T, F>(&self, key: &str, ttl: Option<Duration>, f: F) -> CacheResult<T>
    where
        T: DeserializeOwned + Serialize + Send + Sync,
        F: FnOnce() -> T + Send + Sync,
    {
        if let Some(cached) = self.get_typed::<T>(key).await? {
            return Ok(cached);
        }
        
        let value = f();
        self.set_typed(key, &value, ttl).await?;
        Ok(value)
    }
}

#[async_trait]
impl AdvancedCacheManager for RedisPool {
    async fn get_multi<T: DeserializeOwned + Send + Sync>(&self, keys: &[String]) -> CacheResult<Vec<Option<T>>> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut conn = self.get_connection().await?;
        let results: Vec<Option<String>> = conn.mget(keys).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        
        let mut parsed_results = Vec::with_capacity(results.len());
        for result in results {
            match result {
                Some(json_str) => {
                    let parsed: T = serde_json::from_str(&json_str)
                        .map_err(|e| CacheError::Deserialization(e.to_string()))?;
                    parsed_results.push(Some(parsed));
                }
                None => parsed_results.push(None),
            }
        }
        
        Ok(parsed_results)
    }
    
    async fn set_multi<T: Serialize + Send + Sync>(&self, items: &[(&str, T)], ttl: Option<Duration>) -> CacheResult<()> {
        if items.is_empty() {
            return Ok(());
        }
        
        let mut conn = self.get_connection().await?;
        let mut pipeline = redis::pipe();
        
        for (key, value) in items {
            let serialized = serde_json::to_string(value)
                .map_err(|e| CacheError::Serialization(e.to_string()))?;
            pipeline.set(*key, serialized);
            
            if let Some(ttl_duration) = ttl {
                let seconds = ttl_duration.as_secs() as usize;
                pipeline.expire(*key, seconds);
            }
        }
        
        pipeline.query_async(&mut conn).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        
        Ok(())
    }
    
    async fn hash_set(&self, key: &str, field: &str, value: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.hset(key, field, value).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn hash_get(&self, key: &str, field: &str) -> CacheResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let result: Option<String> = conn.hget(key, field).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn hash_get_all(&self, key: &str) -> CacheResult<std::collections::HashMap<String, String>> {
        let mut conn = self.get_connection().await?;
        let result: std::collections::HashMap<String, String> = conn.hgetall(key).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn hash_delete(&self, key: &str, field: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.hdel(key, field).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn list_push(&self, key: &str, value: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.rpush(key, value).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn list_pop(&self, key: &str) -> CacheResult<Option<String>> {
        let mut conn = self.get_connection().await?;
        let result: Option<String> = conn.lpop(key, None).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn list_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let result: Vec<String> = conn.lrange(key, start, stop).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn set_add(&self, key: &str, value: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.sadd(key, value).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn set_remove(&self, key: &str, value: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.srem(key, value).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn set_members(&self, key: &str) -> CacheResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let result: Vec<String> = conn.smembers(key).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn set_is_member(&self, key: &str, value: &str) -> CacheResult<bool> {
        let mut conn = self.get_connection().await?;
        let result: bool = conn.sismember(key, value).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn sorted_set_add(&self, key: &str, score: f64, value: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.zadd(key, value, score).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn sorted_set_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let result: Vec<String> = conn.zrange(key, start, stop).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn sorted_set_range_by_score(&self, key: &str, min: f64, max: f64) -> CacheResult<Vec<String>> {
        let mut conn = self.get_connection().await?;
        let result: Vec<String> = conn.zrangebyscore(key, min, max).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
    
    async fn publish(&self, channel: &str, message: &str) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.publish(channel, message).await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn subscribe(&self, channels: &[String]) -> CacheResult<redis::aio::PubSub> {
        let mut conn = self.get_connection().await?;
        let mut pubsub = conn.as_pubsub();
        
        for channel in channels {
            pubsub.subscribe(channel).await
                .map_err(|e| CacheError::Redis(e.to_string()))?;
        }
        
        Ok(pubsub)
    }
    
    async fn flush_all(&self) -> CacheResult<()> {
        let mut conn = self.get_connection().await?;
        conn.flushall().await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(())
    }
    
    async fn ping(&self) -> CacheResult<String> {
        let mut conn = self.get_connection().await?;
        let result: String = conn.ping().await
            .map_err(|e| CacheError::Redis(e.to_string()))?;
        Ok(result)
    }
}
