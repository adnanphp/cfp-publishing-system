// backend/src/infrastructure/security/argon2_hasher.rs
use argon2::{
    Argon2, PasswordHash, PasswordHasher as Argon2PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Failed to hash password")]
    HashError,
    #[error("Invalid password")]
    VerificationError,
    #[error("Password is too weak")]
    WeakPassword,
}

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash_password(&self, password: &str) -> Result<String, PasswordError>;
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, PasswordError>;
}

pub struct Argon2Hasher {
    argon2: Argon2<'static>,
}

impl Argon2Hasher {
    pub fn new() -> Self {
        // Configure Argon2 with secure parameters
        let argon2 = Argon2::default();
        Self { argon2 }
    }
    
    pub fn validate_password_strength(password: &str) -> Result<(), PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::WeakPassword);
        }
        
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        
        if !(has_lowercase && has_uppercase && has_digit && has_special) {
            return Err(PasswordError::WeakPassword);
        }
        
        Ok(())
    }
}

#[async_trait]
impl PasswordHasher for Argon2Hasher {
    async fn hash_password(&self, password: &str) -> Result<String, PasswordError> {
        Self::validate_password_strength(password)?;
        
        tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = self.argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_| PasswordError::HashError)?
                .to_string();
            
            Ok(password_hash)
        })
        .await
        .map_err(|_| PasswordError::HashError)?
    }
    
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, PasswordError> {
        tokio::task::spawn_blocking(move || {
            let parsed_hash = PasswordHash::new(hash)
                .map_err(|_| PasswordError::VerificationError)?;
            
            let result = self.argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok();
            
            Ok(result)
        })
        .await
        .map_err(|_| PasswordError::VerificationError)?
    }
}
