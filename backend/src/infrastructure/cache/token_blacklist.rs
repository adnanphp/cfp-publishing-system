use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::{info, warn};

use super::{BasicCache, CacheError, CacheResult, CacheKey, Ttl};

pub struct TokenBlacklist {
    cache_manager: Box<dyn BasicCache>,
    jwt_secret: String,
}

impl TokenBlacklist {
    pub fn new(cache_manager: Box<dyn BasicCache>, jwt_secret: String) -> Self {
        Self {
            cache_manager,
            jwt_secret,
        }
    }
    
    pub async fn blacklist_token(&self, token: &str, reason: Option<String>) -> CacheResult<()> {
        // Decode token to get expiry
        let decoding_key = DecodingKey::from_secret(self.jwt_secret.as_bytes());
        let validation = Validation::default();
        
        let token_data = match decode::<serde_json::Value>(token, &decoding_key, &validation) {
            Ok(data) => data,
            Err(_) => {
                // If token is invalid, we still blacklist it for safety
                warn!("Attempted to blacklist invalid token");
            }
        };
        
        // Calculate TTL based on token expiry
        let ttl = self.calculate_token_ttl(&token_data).unwrap_or(Ttl::LONG);
        
        // Create blacklist entry
        let entry = BlacklistEntry {
            token: token.to_string(),
            blacklisted_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
            reason,
            expires_at: None, // Will be calculated from TTL
        };
        
        // Store in cache
        let key = CacheKey::blacklisted_token(token);
        self.cache_manager
            .set(&key, &entry, Some(ttl))
            .await?;
        
        info!("Blacklisted token with TTL: {:?}", ttl);
        
        Ok(())
    }
    
    pub async fn is_token_blacklisted(&self, token: &str) -> CacheResult<bool> {
        let key = CacheKey::blacklisted_token(token);
        
        match self.cache_manager.get::<BlacklistEntry>(&key).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    pub async fn remove_from_blacklist(&self, token: &str) -> CacheResult<()> {
        let key = CacheKey::blacklisted_token(token);
        self.cache_manager.delete(&key).await?;
        
        info!("Removed token from blacklist: {}", token);
        
        Ok(())
    }
    
    pub async fn cleanup_expired_blacklist_entries(&self) -> CacheResult<u64> {
        // Redis will automatically expire entries based on TTL
        // This method is for manual cleanup if needed
        info!("Blacklist entries are automatically expired by Redis TTL");
        Ok(0)
    }
    
    pub async fn get_blacklist_stats(&self) -> CacheResult<BlacklistStats> {
        // Note: Getting exact count of blacklisted tokens requires SCAN
        // This is a simplified implementation
        warn!("Blacklist statistics are not fully implemented in this simplified version");
        
        Ok(BlacklistStats {
            estimated_count: 0,
            last_cleanup: None,
        })
    }
    
    pub async fn blacklist_user_tokens(&self, user_id: Uuid, reason: &str) -> CacheResult<u64> {
        // This would require storing a mapping of user_id -> tokens
        // For now, this is a placeholder implementation
        warn!("Bulk user token blacklisting is not implemented in this simplified version");
        Ok(0)
    }
    
    fn calculate_token_ttl(&self, token_data: &Option<jsonwebtoken::TokenData<serde_json::Value>>) -> Option<Duration> {
        if let Some(data) = token_data {
            if let Some(exp) = data.claims.get("exp") {
                if let Some(exp_timestamp) = exp.as_i64() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_else(|_| Duration::from_secs(0))
                        .as_secs() as i64;
                    
                    let remaining = exp_timestamp - now;
                    if remaining > 0 {
                        return Some(Duration::from_secs(remaining as u64));
                    }
                }
            }
        }
        
        // Default TTL for tokens without expiry or invalid tokens
        Some(Ttl::LONG)
    }
    
    pub async fn add_to_temp_blacklist(
        &self,
        identifier: &str,
        ttl: Duration,
        reason: &str,
    ) -> CacheResult<()> {
        let key = format!("temp_blacklist:{}", identifier);
        
        let entry = TempBlacklistEntry {
            identifier: identifier.to_string(),
            blacklisted_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_secs(),
            reason: reason.to_string(),
            expires_in: ttl.as_secs(),
        };
        
        self.cache_manager
            .set(&key, &entry, Some(ttl))
            .await?;
        
        info!("Added {} to temporary blacklist for {:?}", identifier, ttl);
        
        Ok(())
    }
    
    pub async fn is_temp_blacklisted(&self, identifier: &str) -> CacheResult<bool> {
        let key = format!("temp_blacklist:{}", identifier);
        
        match self.cache_manager.get::<TempBlacklistEntry>(&key).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct BlacklistEntry {
    token: String,
    blacklisted_at: u64,
    reason: Option<String>,
    expires_at: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TempBlacklistEntry {
    identifier: String,
    blacklisted_at: u64,
    reason: String,
    expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct BlacklistStats {
    pub estimated_count: u64,
    pub last_cleanup: Option<u64>,
}
