// backend/src/config/jwt.rs
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_expiration_minutes: i64,
    pub refresh_token_expiration_days: i64,
    pub issuer: String,
    pub audience: String,
    pub leeway_seconds: u64,
}

impl JwtSettings {
    pub fn access_token_expiration(&self) -> Duration {
        Duration::from_secs((self.access_token_expiration_minutes * 60) as u64)
    }

    pub fn refresh_token_expiration(&self) -> Duration {
        Duration::from_secs((self.refresh_token_expiration_days * 24 * 3600) as u64)
    }

    pub fn leeway(&self) -> Duration {
        Duration::from_secs(self.leeway_seconds)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.access_token_secret.len() < 32 {
            return Err("Access token secret must be at least 32 characters".to_string());
        }
        
        if self.refresh_token_secret.len() < 32 {
            return Err("Refresh token secret must be at least 32 characters".to_string());
        }
        
        if self.access_token_expiration_minutes < 5 {
            return Err("Access token expiration must be at least 5 minutes".to_string());
        }
        
        if self.refresh_token_expiration_days < 1 {
            return Err("Refresh token expiration must be at least 1 day".to_string());
        }
        
        Ok(())
    }
}
