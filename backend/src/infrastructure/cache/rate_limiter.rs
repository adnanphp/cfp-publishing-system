use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{info, warn};

use super::{BasicCache, CacheError, CacheResult, CacheKey};

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded for {}. Limit: {}, Window: {:?}", .identifier, .limit, .window)]
    RateLimitExceeded {
        identifier: String,
        limit: u64,
        window: Duration,
        retry_after: Duration,
    },
    
    #[error("Invalid rate limit configuration")]
    InvalidConfig,
}

pub struct RateLimiter {
    cache_manager: Box<dyn BasicCache>,
}

impl RateLimiter {
    pub fn new(cache_manager: Box<dyn BasicCache>) -> Self {
        Self { cache_manager }
    }
    
    pub async fn check_rate_limit(
        &self,
        identifier: &str,
        limit: u64,
        window: Duration,
    ) -> Result<RateLimitInfo, RateLimitError> {
        let window_key = RateLimiter::get_window_key(identifier, window);
        let count_key = RateLimiter::get_count_key(identifier, window);
        
        // Use Redis INCR to atomically increment and get count
        let current_count: u64 = match self.cache_manager.increment(&count_key, 1).await {
            Ok(count) => count,
            Err(_) => {
                // If Redis fails, we should fail open (allow the request)
                // but log the error
                warn!("Redis failed during rate limiting for {}", identifier);
                return Ok(RateLimitInfo {
                    identifier: identifier.to_string(),
                    current_count: 0,
                    limit,
                    window,
                    remaining: limit,
                    reset_time: SystemTime::now() + window,
                    is_exceeded: false,
                });
            }
        };
        
        // If this is the first request in this window, set expiry
        if current_count == 1 {
            if let Err(e) = self.cache_manager.expire(&count_key, window).await {
                warn!("Failed to set expiry for rate limit key: {}", e);
            }
        }
        
        // Check if limit is exceeded
        let is_exceeded = current_count > limit;
        let remaining = if current_count > limit { 0 } else { limit - current_count };
        
        // Calculate reset time
        let reset_time = if let Ok(Some(ttl)) = self.cache_manager.get::<i64>(&window_key).await {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0));
            SystemTime::UNIX_EPOCH + now + Duration::from_secs(ttl as u64)
        } else {
            SystemTime::now() + window
        };
        
        if is_exceeded {
            let retry_after = reset_time.duration_since(SystemTime::now())
                .unwrap_or(Duration::from_secs(0));
            
            return Err(RateLimitError::RateLimitExceeded {
                identifier: identifier.to_string(),
                limit,
                window,
                retry_after,
            });
        }
        
        Ok(RateLimitInfo {
            identifier: identifier.to_string(),
            current_count,
            limit,
            window,
            remaining,
            reset_time,
            is_exceeded,
        })
    }
    
    pub async fn check_rate_limit_with_cost(
        &self,
        identifier: &str,
        limit: u64,
        window: Duration,
        cost: u64,
    ) -> Result<RateLimitInfo, RateLimitError> {
        if cost == 0 {
            return self.check_rate_limit(identifier, limit, window).await;
        }
        
        if cost > limit {
            return Err(RateLimitError::InvalidConfig);
        }
        
        let count_key = RateLimiter::get_count_key(identifier, window);
        
        // Get current count
        let current_count: u64 = match self.cache_manager.get(&count_key).await {
            Ok(Some(count)) => count,
            Ok(None) => 0,
            Err(_) => {
                warn!("Redis failed during rate limiting for {}", identifier);
                return Ok(RateLimitInfo {
                    identifier: identifier.to_string(),
                    current_count: 0,
                    limit,
                    window,
                    remaining: limit,
                    reset_time: SystemTime::now() + window,
                    is_exceeded: false,
                });
            }
        };
        
        // Check if adding cost would exceed limit
        let new_count = current_count + cost;
        let is_exceeded = new_count > limit;
        
        if is_exceeded {
            let reset_time = if let Ok(Some(ttl)) = self.cache_manager.get::<i64>(&count_key).await {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| Duration::from_secs(0));
                SystemTime::UNIX_EPOCH + now + Duration::from_secs(ttl as u64)
            } else {
                SystemTime::now() + window
            };
            
            let retry_after = reset_time.duration_since(SystemTime::now())
                .unwrap_or(Duration::from_secs(0));
            
            return Err(RateLimitError::RateLimitExceeded {
                identifier: identifier.to_string(),
                limit,
                window,
                retry_after,
            });
        }
        
        // Increment by cost
        match self.cache_manager.increment(&count_key, cost as i64).await {
            Ok(final_count) => {
                // If this was the first increment in this window, set expiry
                if current_count == 0 {
                    if let Err(e) = self.cache_manager.expire(&count_key, window).await {
                        warn!("Failed to set expiry for rate limit key: {}", e);
                    }
                }
                
                let remaining = if final_count > limit { 0 } else { limit - final_count };
                let reset_time = SystemTime::now() + window;
                
                Ok(RateLimitInfo {
                    identifier: identifier.to_string(),
                    current_count: final_count,
                    limit,
                    window,
                    remaining,
                    reset_time,
                    is_exceeded: false,
                })
            }
            Err(_) => {
                warn!("Redis failed during rate limiting for {}", identifier);
                Ok(RateLimitInfo {
                    identifier: identifier.to_string(),
                    current_count: 0,
                    limit,
                    window,
                    remaining: limit,
                    reset_time: SystemTime::now() + window,
                    is_exceeded: false,
                })
            }
        }
    }
    
    pub async fn get_rate_limit_info(
        &self,
        identifier: &str,
        window: Duration,
    ) -> Result<RateLimitInfo, CacheError> {
        let count_key = RateLimiter::get_count_key(identifier, window);
        
        let current_count: u64 = match self.cache_manager.get(&count_key).await {
            Ok(Some(count)) => count,
            Ok(None) => 0,
            Err(e) => return Err(e),
        };
        
        // For this method, we need to know the limit from configuration
        // This is a simplified version - in reality, you'd get this from config
        let limit = 100; // Default limit
        
        let remaining = if current_count > limit { 0 } else { limit - current_count };
        let is_exceeded = current_count > limit;
        
        let reset_time = if let Ok(Some(ttl)) = self.cache_manager.get::<i64>(&count_key).await {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0));
            SystemTime::UNIX_EPOCH + now + Duration::from_secs(ttl as u64)
        } else {
            SystemTime::now() + window
        };
        
        Ok(RateLimitInfo {
            identifier: identifier.to_string(),
            current_count,
            limit,
            window,
            remaining,
            reset_time,
            is_exceeded,
        })
    }
    
    pub async fn reset_rate_limit(&self, identifier: &str, window: Duration) -> Result<(), CacheError> {
        let count_key = RateLimiter::get_count_key(identifier, window);
        self.cache_manager.delete(&count_key).await
    }
    
    pub async fn cleanup_expired_rate_limits(&self) -> Result<u64, CacheError> {
        // Redis will automatically expire rate limit keys based on TTL
        info!("Rate limit keys are automatically expired by Redis TTL");
        Ok(0)
    }
    
    fn get_window_key(identifier: &str, window: Duration) -> String {
        let window_str = format!("{}s", window.as_secs());
        CacheKey::rate_limit(identifier, &window_str)
    }
    
    fn get_count_key(identifier: &str, window: Duration) -> String {
        format!("{}:count", Self::get_window_key(identifier, window))
    }
    
    // Pre-configured rate limiters for common use cases
    pub async fn check_auth_rate_limit(&self, ip_address: &str) -> Result<RateLimitInfo, RateLimitError> {
        // 10 attempts per minute for authentication
        self.check_rate_limit(&format!("auth:{}", ip_address), 10, Duration::from_secs(60)).await
    }
    
    pub async fn check_api_rate_limit(&self, api_key: &str) -> Result<RateLimitInfo, RateLimitError> {
        // 1000 requests per hour per API key
        self.check_rate_limit(&format!("api:{}", api_key), 1000, Duration::from_secs(3600)).await
    }
    
    pub async fn check_download_rate_limit(&self, member_id: &str) -> Result<RateLimitInfo, RateLimitError> {
        // 10 downloads per day for non-donors, 100 for donors
        // This is simplified - actual logic would check member status
        self.check_rate_limit(&format!("download:{}", member_id), 10, Duration::from_secs(86400)).await
    }
    
    pub async fn check_comment_rate_limit(&self, member_id: &str) -> Result<RateLimitInfo, RateLimitError> {
        // 5 comments per hour
        self.check_rate_limit(&format!("comment:{}", member_id), 5, Duration::from_secs(3600)).await
    }
    
    pub async fn check_vote_rate_limit(&self, member_id: &str) -> Result<RateLimitInfo, RateLimitError> {
        // 10 votes per day
        self.check_rate_limit(&format!("vote:{}", member_id), 10, Duration::from_secs(86400)).await
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub identifier: String,
    pub current_count: u64,
    pub limit: u64,
    pub window: Duration,
    pub remaining: u64,
    pub reset_time: SystemTime,
    pub is_exceeded: bool,
}

impl RateLimitInfo {
    pub fn to_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        
        headers.push(("X-RateLimit-Limit".to_string(), self.limit.to_string()));
        headers.push(("X-RateLimit-Remaining".to_string(), self.remaining.to_string()));
        
        if let Ok(duration) = self.reset_time.duration_since(SystemTime::now()) {
            let reset_timestamp = self.reset_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs();
            
            headers.push(("X-RateLimit-Reset".to_string(), reset_timestamp.to_string()));
            headers.push(("Retry-After".to_string(), duration.as_secs().to_string()));
        }
        
        if self.is_exceeded {
            headers.push(("X-RateLimit-Exceeded".to_string(), "true".to_string()));
        }
        
        headers
    }
}

// Sliding window rate limiter for more precise rate limiting
pub struct SlidingWindowRateLimiter {
    cache_manager: Box<dyn BasicCache>,
}

impl SlidingWindowRateLimiter {
    pub fn new(cache_manager: Box<dyn BasicCache>) -> Self {
        Self { cache_manager }
    }
    
    pub async fn check_sliding_window(
        &self,
        identifier: &str,
        limit: u64,
        window: Duration,
    ) -> Result<SlidingWindowRateLimitInfo, RateLimitError> {
        // This implements a more sophisticated sliding window algorithm
        // using Redis sorted sets
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs_f64();
        
        let window_start = now - window.as_secs_f64();
        let key = format!("sliding:{}:{}", identifier, window.as_secs());
        
        // Remove old entries
        let _: () = self.cache_manager
            .sorted_set_range_by_score(&key, 0.0, window_start)
            .await
            .map_err(|_| RateLimitError::InvalidConfig)?;
        
        // Add current request
        self.cache_manager
            .sorted_set_add(&key, now, &now.to_string())
            .await
            .map_err(|_| RateLimitError::InvalidConfig)?;
        
        // Get count in window
        let count = self.cache_manager
            .sorted_set_range_by_score(&key, window_start, now)
            .await
            .map_err(|_| RateLimitError::InvalidConfig)?
            .len() as u64;
        
        // Set expiry on the key
        self.cache_manager
            .expire(&key, window * 2)
            .await
            .map_err(|_| RateLimitError::InvalidConfig)?;
        
        if count > limit {
            // Find oldest request to calculate retry time
            let oldest = self.cache_manager
                .sorted_set_range(&key, 0, 0)
                .await
                .map_err(|_| RateLimitError::InvalidConfig)?;
            
            let retry_after = if let Some(oldest_str) = oldest.first() {
                if let Ok(oldest_time) = oldest_str.parse::<f64>() {
                    Duration::from_secs_f64((oldest_time + window.as_secs_f64()) - now).max(Duration::from_secs(0))
                } else {
                    window
                }
            } else {
                window
            };
            
            return Err(RateLimitError::RateLimitExceeded {
                identifier: identifier.to_string(),
                limit,
                window,
                retry_after,
            });
        }
        
        Ok(SlidingWindowRateLimitInfo {
            identifier: identifier.to_string(),
            current_count: count,
            limit,
            window,
            remaining: limit - count,
            window_start: window_start as u64,
            is_exceeded: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SlidingWindowRateLimitInfo {
    pub identifier: String,
    pub current_count: u64,
    pub limit: u64,
    pub window: Duration,
    pub remaining: u64,
    pub window_start: u64,
    pub is_exceeded: bool,
}
