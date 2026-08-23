#!/bin/bash

echo "Creating minimal working setup..."

# 1. First, backup the current src directory
echo "Backing up current source..."
cp -r src src.backup.$(date +%s)

# 2. Create a minimal working structure
echo "Creating minimal structure..."
rm -rf src
mkdir -p src

# Create the most basic working structure
cat > src/lib.rs << 'EOF'
pub mod api;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
EOF

# Create minimal API structure
mkdir -p src/api/{handlers,middleware,routes}
cat > src/api/mod.rs << 'EOF'
pub mod handlers;
pub mod middleware;
pub mod routes;
EOF

# Create basic handlers
cat > src/api/handlers/mod.rs << 'EOF'
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
use actix_web::{HttpResponse, Responder};
use crate::api::responses::ApiResponse;

pub async fn placeholder() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("Placeholder endpoint"))
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

# Update api/mod.rs
echo -e "\npub mod responses;" >> src/api/mod.rs

# 3. Create minimal domain structure
mkdir -p src/domain/{models,value_objects,aggregates,repositories,enums}
cat > src/domain/mod.rs << 'EOF'
pub mod models;
pub mod value_objects;
pub mod aggregates;
pub mod repositories;
pub mod enums;
EOF

# Create basic models
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

# 4. Create minimal application structure
mkdir -p src/application/{dto,services,queries,commands}
cat > src/application/mod.rs << 'EOF'
pub mod dto;
pub mod services;
pub mod queries;
pub mod commands;
EOF

# Create basic DTOs
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

# 5. Create minimal infrastructure structure
mkdir -p src/infrastructure/{database,cache,security,messaging,external}
cat > src/infrastructure/mod.rs << 'EOF'
pub mod database;
pub mod cache;
pub mod security;
pub mod messaging;
pub mod external;
EOF

# 6. Create minimal utils
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

# 7. Update Cargo.toml with minimal dependencies
echo "Updating Cargo.toml..."
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

# 8. Create a simple main.rs for testing
cat > src/main.rs << 'EOF'
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use cfp_backend::api::handlers;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting CFP Backend server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
EOF

# 9. Now let's restore some of the original structure gradually
echo "Restoring original structure gradually..."

# Restore only essential files from backup
if [ -d "src.backup" ]; then
    # Copy back routes if they exist
    if [ -d "src.backup/api/routes" ]; then
        cp -r src.backup/api/routes/* src/api/routes/ 2>/dev/null || true
    fi
    
    # Copy back specific important files
    important_files=(
        "src.backup/domain/enums/mod.rs"
        "src.backup/domain/repositories/mod.rs"
        "src.backup/application/services/mod.rs"
        "src.backup/infrastructure/database/mod.rs"
    )
    
    for file in "${important_files[@]}"; do
        if [ -f "$file" ]; then
            dir=$(dirname "${file/src.backup/src}")
            mkdir -p "$dir"
            cp "$file" "${file/src.backup/src}"
        fi
    done
fi

echo "Minimal setup created. Running cargo check..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Compilation successful! Now you can gradually add back functionality."
    echo ""
    echo "Next steps:"
    echo "1. Test with: cargo run"
    echo "2. Gradually copy files back from src.backup/"
    echo "3. Run 'cargo check' after each addition"
    echo "4. Fix errors as they appear"
else
    echo "❌ Compilation failed. Let's see what's wrong..."
    echo "Running cargo check with more details..."
    cargo check 2>&1 | head -50
fi
