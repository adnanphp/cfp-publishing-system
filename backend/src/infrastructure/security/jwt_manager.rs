// backend/src/infrastructure/security/jwt_manager.rs
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

use crate::config::Settings;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Failed to create token")]
    CreationError,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    ExpiredToken,
    #[error("Invalid token type")]
    InvalidTokenType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // Subject (member_id)
    pub exp: usize, // Expiration
    pub iat: usize, // Issued at
    pub jti: String, // JWT ID
    pub typ: String, // Token type: "access" or "refresh"
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_at: SystemTime,
    pub refresh_token_expires_at: SystemTime,
}

pub struct JwtManager {
    access_key: EncodingKey,
    refresh_key: EncodingKey,
    decoding_key_access: DecodingKey,
    decoding_key_refresh: DecodingKey,
    settings: Settings,
}

impl JwtManager {
    pub fn new(settings: Settings) -> Result<Self, JwtError> {
        settings.validate()
            .map_err(|_| JwtError::CreationError)?;
        
        let access_key = EncodingKey::from_secret(settings.access_token_secret.as_bytes());
        let refresh_key = EncodingKey::from_secret(settings.refresh_token_secret.as_bytes());
        
        let decoding_key_access = DecodingKey::from_secret(settings.access_token_secret.as_bytes());
        let decoding_key_refresh = DecodingKey::from_secret(settings.refresh_token_secret.as_bytes());
        
        Ok(Self {
            access_key,
            refresh_key,
            decoding_key_access,
            decoding_key_refresh,
            settings,
        })
    }
    
    pub fn generate_token_pair(
        &self,
        member_id: i32,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<TokenPair, JwtError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| JwtError::CreationError)?
            .as_secs() as usize;
        
        // Access token
        let access_exp = now + (self.settings.access_token_expiration_minutes * 60) as usize;
        let access_claims = TokenClaims {
            sub: member_id.to_string(),
            exp: access_exp,
            iat: now,
            jti: Uuid::new_v4().to_string(),
            typ: "access".to_string(),
            roles: roles.clone(),
            permissions: permissions.clone(),
        };
        
        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &access_claims,
            &self.access_key,
        )
        .map_err(|_| JwtError::CreationError)?;
        
        // Refresh token
        let refresh_exp = now + (self.settings.refresh_token_expiration_days * 24 * 3600) as usize;
        let refresh_claims = TokenClaims {
            sub: member_id.to_string(),
            exp: refresh_exp,
            iat: now,
            jti: Uuid::new_v4().to_string(),
            typ: "refresh".to_string(),
            roles,
            permissions,
        };
        
        let refresh_token = encode(
            &Header::new(Algorithm::HS256),
            &refresh_claims,
            &self.refresh_key,
        )
        .map_err(|_| JwtError::CreationError)?;
        
        let access_token_expires_at = UNIX_EPOCH + std::time::Duration::from_secs(access_exp as u64);
        let refresh_token_expires_at = UNIX_EPOCH + std::time::Duration::from_secs(refresh_exp as u64);
        
        Ok(TokenPair {
            access_token,
            refresh_token,
            access_token_expires_at,
            refresh_token_expires_at,
        })
    }
    
    pub fn validate_access_token(&self, token: &str) -> Result<TokenClaims, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = self.settings.leeway_seconds;
        validation.validate_exp = true;
        validation.set_issuer(&[self.settings.issuer.clone()]);
        validation.set_audience(&[self.settings.audience.clone()]);
        
        let token_data = decode::<TokenClaims>(
            token,
            &self.decoding_key_access,
            &validation,
        )
        .map_err(|e| {
            tracing::error!("Token validation error: {}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
                _ => JwtError::InvalidToken,
            }
        })?;
        
        if token_data.claims.typ != "access" {
            return Err(JwtError::InvalidTokenType);
        }
        
        Ok(token_data.claims)
    }
    
    pub fn validate_refresh_token(&self, token: &str) -> Result<TokenClaims, JwtError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = self.settings.leeway_seconds;
        validation.validate_exp = true;
        validation.set_issuer(&[self.settings.issuer.clone()]);
        validation.set_audience(&[self.settings.audience.clone()]);
        
        let token_data = decode::<TokenClaims>(
            token,
            &self.decoding_key_refresh,
            &validation,
        )
        .map_err(|e| {
            tracing::error!("Refresh token validation error: {}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredToken,
                _ => JwtError::InvalidToken,
            }
        })?;
        
        if token_data.claims.typ != "refresh" {
            return Err(JwtError::InvalidTokenType);
        }
        
        Ok(token_data.claims)
    }
    
    pub fn refresh_access_token(
        &self,
        refresh_token: &str,
        new_roles: Vec<String>,
        new_permissions: Vec<String>,
    ) -> Result<TokenPair, JwtError> {
        let claims = self.validate_refresh_token(refresh_token)?;
        let member_id: i32 = claims.sub.parse()
            .map_err(|_| JwtError::InvalidToken)?;
        
        self.generate_token_pair(member_id, new_roles, new_permissions)
    }
}
