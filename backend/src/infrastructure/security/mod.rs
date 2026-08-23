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
