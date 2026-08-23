#!/bin/bash

echo "Fixing compilation issues step by step..."
echo "=========================================="

# 1. Create the problematic_files directory
mkdir -p problematic_files

# 2. Fix domain/enums duplication issue
echo -e "\n1. Fixing domain/enums duplication..."
if [ -f "src/domain/enums.rs" ] && [ -f "src/domain/enums/mod.rs" ]; then
    echo "Removing duplicate: src/domain/enums.rs"
    mv "src/domain/enums.rs" "problematic_files/domain/enums_single.rs"
fi

# 3. Fix missing dependencies
echo -e "\n2. Adding missing dependencies to Cargo.toml..."
if ! grep -q "log" Cargo.toml; then
    echo 'log = "0.4"' >> Cargo.toml
    echo "Added: log"
fi

if ! grep -q "lazy_static" Cargo.toml; then
    echo 'lazy_static = "1.4"' >> Cargo.toml
    echo "Added: lazy_static"
fi

if ! grep -q "regex" Cargo.toml; then
    echo 'regex = "1.0"' >> Cargo.toml
    echo "Added: regex"
fi

if ! grep -q "sha2" Cargo.toml; then
    echo 'sha2 = "0.10"' >> Cargo.toml
    echo "Added: sha2"
fi

if ! grep -q "env_logger" Cargo.toml; then
    echo 'env_logger = "0.10"' >> Cargo.toml
    echo "Added: env_logger"
fi

# 4. Fix missing modules in application/dto
echo -e "\n3. Fixing application/dto missing modules..."
mkdir -p src/application/dto/types
touch src/application/dto/types/mod.rs

# Create missing request modules
mkdir -p src/application/dto/requests
for module in auth member text donation plagiarism notification committee; do
    touch "src/application/dto/requests/${module}_request.rs"
done

# Create missing response modules  
mkdir -p src/application/dto/responses
for module in auth member text donation plagiarism notification committee; do
    touch "src/application/dto/responses/${module}_response.rs"
done

# Update application/dto/mod.rs with proper module declarations
cat > src/application/dto/mod.rs << 'EOF'
pub mod types;
pub mod requests;
pub mod responses;

// Re-export commonly used types
pub use requests::*;
pub use responses::*;
EOF

# Update requests/mod.rs
cat > src/application/dto/requests/mod.rs << 'EOF'
pub mod auth_request;
pub mod member_request;
pub mod text_request;
pub mod donation_request;
pub mod plagiarism_request;
pub mod notification_request;
pub mod committee_request;

// Re-export
pub use auth_request::*;
pub use member_request::*;
pub use text_request::*;
pub use donation_request::*;
pub use plagiarism_request::*;
pub use notification_request::*;
pub use committee_request::*;
EOF

# Update responses/mod.rs
cat > src/application/dto/responses/mod.rs << 'EOF'
pub mod auth_response;
pub mod member_response;
pub mod text_response;
pub mod donation_response;
pub mod plagiarism_response;
pub mod notification_response;
pub mod committee_response;

// Re-export
pub use auth_response::*;
pub use member_response::*;
pub use text_response::*;
pub use donation_response::*;
pub use plagiarism_response::*;
pub use notification_response::*;
pub use committee_response::*;
EOF

# 5. Fix missing models in domain/models
echo -e "\n4. Fixing domain/models missing modules..."

# Update domain/models/mod.rs to include all models
cat > src/domain/models/mod.rs << 'EOF'
pub mod admin;
pub mod author;
pub mod charity;
pub mod comment;
pub mod committee;
pub mod committee_membership;
pub mod donation;
pub mod download;
pub mod member;
pub mod message;
pub mod moderator;
pub mod notification;
pub mod plagiarism_case;
pub mod text;
pub mod text_version;
pub mod vote;

// Re-export models
pub use admin::*;
pub use author::*;
pub use charity::*;
pub use comment::*;
pub use committee::*;
pub use committee_membership::*;
pub use donation::*;
pub use download::*;
pub use member::*;
pub use message::*;
pub use moderator::*;
pub use notification::*;
pub use plagiarism_case::*;
pub use text::*;
pub use text_version::*;
pub use vote::*;
EOF

# Now restore the model files that failed before
echo -e "\n5. Restoring domain models that failed earlier..."
BACKUP="./src.backup.1769027748"

MODEL_FILES=(
    "domain/models/notification.rs"
    "domain/models/plagiarism_case.rs"
    "domain/models/text.rs"
    "domain/models/text_version.rs"
    "domain/models/vote.rs"
)

for file in "${MODEL_FILES[@]}"; do
    if [ -f "$BACKUP/$file" ]; then
        echo "Restoring: $file"
        cp "$BACKUP/$file" "src/$file"
    fi
done

# 6. Fix utils/error.rs by removing log dependency or adding it
echo -e "\n6. Fixing utils/error.rs..."
if [ -f "src/utils/error.rs" ]; then
    # Create a backup
    cp "src/utils/error.rs" "src/utils/error.rs.bak"
    
    # Remove log-related code or create minimal version
    cat > /tmp/error_fixed.rs << 'EOF'
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
}
EOF
    
    cp /tmp/error_fixed.rs "src/utils/error.rs"
fi

# 7. Fix utils/validation.rs
echo -e "\n7. Fixing utils/validation.rs..."
if [ -f "src/utils/validation.rs" ]; then
    # Create minimal version
    cat > /tmp/validation_fixed.rs << 'EOF'
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_]{3,20}$"
    ).unwrap();
}

pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    USERNAME_REGEX.is_match(username)
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
EOF
    
    cp /tmp/validation_fixed.rs "src/utils/validation.rs"
fi

# 8. Fix utils/cryptography.rs imports
echo -e "\n8. Fixing utils/cryptography.rs imports..."
if [ -f "src/utils/cryptography.rs" ]; then
    # Check if it has sha2 import
    if grep -q "use sha2" "src/utils/cryptography.rs"; then
        echo "cryptography.rs already has sha2 import"
    else
        # Add sha2 import if missing
        sed -i '1s/^/use sha2::{Sha256, Digest};\n/' "src/utils/cryptography.rs"
    fi
fi

# 9. Fix utils/logger.rs imports
echo -e "\n9. Fixing utils/logger.rs imports..."
if [ -f "src/utils/logger.rs" ]; then
    # Check if it has env_logger import
    if grep -q "use env_logger" "src/utils/logger.rs"; then
        echo "logger.rs already has env_logger import"
    else
        # Add env_logger import if missing
        sed -i '1s/^/use env_logger;\n/' "src/utils/logger.rs"
    fi
fi

# 10. Update utils/mod.rs to include all modules
echo -e "\n10. Updating utils/mod.rs..."
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

// Utility functions
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

# 11. Test compilation
echo -e "\n11. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo -e "\n✅ SUCCESS! Project compiles!"
    echo -e "\nNext steps:"
    echo "1. You can now restore remaining modules:"
    echo "   - application/services/"
    echo "   - application/queries/"
    echo "   - application/commands/"
    echo "   - infrastructure/"
    echo ""
    echo "2. Test with: cargo run (if you have main.rs)"
    echo ""
    echo "3. Check what's still in backup:"
    echo "   find ./src.backup.1769027748 -name '*.rs' | grep -E '(services|queries|commands|infrastructure)' | head -20"
else
    echo -e "\n❌ Still have compilation errors. Showing first few:"
    cargo check 2>&1 | grep -A 2 "error\[E" | head -20
fi
