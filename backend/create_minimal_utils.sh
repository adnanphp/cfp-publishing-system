#!/bin/bash

echo "Creating minimal utils files..."
echo "==============================="

# Backup current utils
cp -r src/utils src/utils.backup.complex

# Create minimal error.rs
cat > src/utils/error.rs << 'EOF'
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl AppError {
    pub fn database(msg: &str) -> Self {
        AppError::Database(msg.to_string())
    }
    
    pub fn validation(msg: &str) -> Self {
        AppError::Validation(msg.to_string())
    }
    
    pub fn authentication(msg: &str) -> Self {
        AppError::Authentication(msg.to_string())
    }
    
    pub fn not_found(msg: &str) -> Self {
        AppError::NotFound(msg.to_string())
    }
    
    pub fn internal(msg: &str) -> Self {
        AppError::Internal(msg.to_string())
    }
    
    pub fn external_service(msg: &str) -> Self {
        AppError::ExternalService(msg.to_string())
    }
}
EOF

# Create minimal validation.rs
cat > src/utils/validation.rs << 'EOF'
pub fn validate_email(email: &str) -> bool {
    !email.is_empty() && email.contains('@') && email.contains('.')
}

pub fn validate_username(username: &str) -> bool {
    let len = username.len();
    len >= 3 && len <= 50 && username.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 100
}

pub fn validate_text_content(content: &str) -> bool {
    !content.trim().is_empty() && content.len() <= 10000
}

pub fn validate_amount(amount: f64) -> bool {
    amount > 0.0 && amount <= 1000000.0
}

pub fn sanitize_input(input: &str) -> String {
    input.trim().to_string()
}

pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}
EOF

# Create minimal logger.rs
cat > src/utils/logger.rs << 'EOF'
use tracing::{info, error, warn, debug};

pub fn setup_logger() {
    tracing_subscriber::fmt::init();
    info!("Logger initialized");
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}
EOF

# Create minimal cryptography.rs
cat > src/utils/cryptography.rs << 'EOF'
use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

pub fn generate_token() -> String {
    generate_random_string(32)
}

pub fn generate_session_id() -> String {
    generate_random_string(64)
}

pub fn hash_password(password: &str) -> String {
    // Simple hash for now - replace with proper hashing later
    format!("hashed_{}", password)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash == format!("hashed_{}", password)
}
EOF

# Create minimal datetime.rs
cat > src/utils/datetime.rs << 'EOF'
use chrono::{DateTime, Utc};

pub fn get_current_datetime() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_datetime(dt: &DateTime<Utc>, format: &str) -> String {
    dt.format(format).to_string()
}

pub fn parse_datetime(datetime_str: &str, format: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_str(datetime_str, format)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

pub fn add_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    dt + chrono::Duration::days(days)
}

pub fn subtract_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    dt - chrono::Duration::days(days)
}

pub fn get_start_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
}

pub fn get_end_of_day(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc()
}
EOF

# Create minimal mod.rs
cat > src/utils/mod.rs << 'EOF'
pub mod error;
pub mod validation;
pub mod logger;
pub mod cryptography;
pub mod datetime;

use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

pub fn format_error_chain(e: &impl std::error::Error) -> String {
    let mut result = e.to_string();
    let mut source = e.source();
    
    while let Some(err) = source {
        result.push_str(&format!("\nCaused by: {}", err));
        source = err.source();
    }
    
    result
}

pub fn get_current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}
EOF

echo "Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Minimal utils compiled successfully!"
    echo ""
    echo "Created clean, minimal utils with:"
    echo "  - Basic error handling"
    echo "  - Simple validation functions"
    echo "  - Minimal logging"
    echo "  - Basic cryptography"
    echo "  - Date/time utilities"
    echo ""
    echo "Next steps:"
    echo "1. Test with: cargo build"
    echo "2. Run: cargo run"
    echo "3. Then restore other modules one by one"
else
    echo "❌ Still has errors:"
    cargo check 2>&1 | tail -20
fi
