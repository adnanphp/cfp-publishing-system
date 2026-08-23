#!/bin/bash

echo "Step-by-step project rebuild..."
echo "================================"

# Step 1: Add serde_yaml dependency
echo "Step 1: Adding missing dependencies..."
if ! grep -q "serde_yaml" Cargo.toml; then
    echo 'serde_yaml = "0.9"' >> Cargo.toml
    echo "Added serde_yaml"
fi

# Step 2: Create basic module structure
echo -e "\nStep 2: Creating basic module structure..."

# Update lib.rs to include modules
cat > src/lib.rs << 'EOF'
pub mod api;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod utils;

pub fn hello() -> String {
    "Hello from CFP Backend".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
EOF

# Step 3: Create API module
echo -e "\nStep 3: Creating API module..."
mkdir -p src/api
cat > src/api/mod.rs << 'EOF'
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod responses;
EOF

# Create handlers module
mkdir -p src/api/handlers
cat > src/api/handlers/mod.rs << 'EOF'
// Handlers module
pub mod auth_handler;
pub mod text_handler;
pub mod donation_handler;
pub mod plagiarism_handler;
pub mod notification_handler;
pub mod member_handler;
pub mod committee_handler;
EOF

# Create placeholder handlers
for handler in auth text donation plagiarism notification member committee; do
    cat > src/api/handlers/${handler}_handler.rs << EOF
use crate::api::responses::ApiResponse;
use actix_web::{HttpResponse, Responder};

pub async fn placeholder() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("Placeholder"))
}
EOF
done

# Create responses module
cat > src/api/responses.rs << 'EOF'
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
EOF

# Create empty middleware and routes
touch src/api/middleware.rs
touch src/api/routes.rs

# Step 4: Create domain module
echo -e "\nStep 4: Creating Domain module..."
mkdir -p src/domain
cat > src/domain/mod.rs << 'EOF'
pub mod models;
pub mod value_objects;
pub mod aggregates;
pub mod repositories;
pub mod enums;
EOF

# Create basic models
mkdir -p src/domain/models
cat > src/domain/models/mod.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Donation {
    pub id: Uuid,
    pub amount: f64,
    pub donor_id: Uuid,
}
EOF

# Create empty domain modules
touch src/domain/value_objects.rs
touch src/domain/aggregates.rs
touch src/domain/repositories.rs
touch src/domain/enums.rs

# Step 5: Create application module
echo -e "\nStep 5: Creating Application module..."
mkdir -p src/application
cat > src/application/mod.rs << 'EOF'
pub mod dto;
pub mod services;
pub mod queries;
pub mod commands;
EOF

# Create DTO structure
mkdir -p src/application/dto
cat > src/application/dto/mod.rs << 'EOF'
pub mod requests;
pub mod responses;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}
EOF

mkdir -p src/application/dto/{requests,responses}
touch src/application/dto/requests/mod.rs
touch src/application/dto/responses/mod.rs

# Create empty application modules
touch src/application/services.rs
touch src/application/queries.rs
touch src/application/commands.rs

# Step 6: Create infrastructure module
echo -e "\nStep 6: Creating Infrastructure module..."
mkdir -p src/infrastructure
cat > src/infrastructure/mod.rs << 'EOF'
pub mod database;
pub mod cache;
pub mod security;
pub mod messaging;
pub mod external;
EOF

# Create empty infrastructure modules
touch src/infrastructure/database.rs
touch src/infrastructure/cache.rs
touch src/infrastructure/security.rs
touch src/infrastructure/messaging.rs
touch src/infrastructure/external.rs

# Step 7: Create utils module
echo -e "\nStep 7: Creating Utils module..."
mkdir -p src/utils
cat > src/utils/mod.rs << 'EOF'
pub mod error;
pub mod validation;

use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

// Remove the serde_yaml functions for now
// We'll add them back when we add the dependency
EOF

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
}
EOF

cat > src/utils/validation.rs << 'EOF'
pub fn validate_email(email: &str) -> bool {
    !email.is_empty() && email.contains('@')
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}
EOF

# Step 8: Update Cargo.toml with all required dependencies
echo -e "\nStep 8: Updating Cargo.toml..."
cat > Cargo.toml << 'EOF'
[package]
name = "cfp-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
actix-rt = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
rand = "0.8"
async-trait = "0.1"
tracing = "0.1"
tracing-subscriber = "0.3"

[dependencies.redis]
version = "0.22"
features = ["tokio-comp"]

[dependencies.sqlx]
version = "0.7"
features = ["postgres", "runtime-tokio-native-tls", "macros", "chrono", "uuid", "migrate"]

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
EOF

# Step 9: Test compilation
echo -e "\nStep 9: Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo -e "\n✅ Success! Basic structure compiles."
    echo -e "\nNext steps:"
    echo "1. Copy specific files from your backup one by one"
    echo "2. After copying each file, run: cargo check"
    echo "3. Fix any errors before proceeding"
    echo ""
    echo "Example to copy routes:"
    echo "  cp /path/to/backup/api/routes/mod.rs src/api/routes/"
    echo "  cargo check"
    echo ""
    echo "If you have a backup directory, list files to copy:"
    echo "  find /path/to/backup -name \"*.rs\" | head -20"
else
    echo -e "\n❌ Compilation failed. Here are the first errors:"
    cargo check 2>&1 | grep -A 3 "error\[E"
fi
