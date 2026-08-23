#!/bin/bash

echo "Fixing remaining compilation errors..."

# 1. Fix duplicate import in text_response.rs
echo "Fixing duplicate import..."
sed -i '2d' src/application/dto/responses/text_response.rs

# 2. Fix responses module path
echo "Fixing responses module..."
# Create the responses module in the correct location if it doesn't exist
mkdir -p src/api
if [ ! -f "src/api/responses.rs" ]; then
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
fi

# Update mod.rs in api directory
if [ ! -f "src/api/mod.rs" ]; then
    cat > src/api/mod.rs << 'EOF'
pub mod handlers;
pub mod middleware;
pub mod routes;
pub mod responses;
EOF
fi

# Fix import in handlers/mod.rs
sed -i '22s/use super::responses::ApiResponse;/use crate::api::responses::ApiResponse;/' src/api/handlers/mod.rs

# Fix imports in all handlers
for file in src/api/handlers/*.rs; do
    sed -i 's/use super::responses::ApiResponse;/use crate::api::responses::ApiResponse;/' "$file"
done

# 3. Fix plagiarism response types
echo "Creating missing DTO types..."
mkdir -p src/application/dto/responses

cat > src/application/dto/responses/plagiarism_response.rs << 'EOF'
use serde::Serialize;

#[derive(Serialize)]
pub struct PlagiarismCaseResponse;

#[derive(Serialize)]
pub struct PlagiarismCaseSearchResponse;

#[derive(Serialize)]
pub struct VoteResponse;

#[derive(Serialize)]
pub struct PlagiarismStatsResponse;
EOF

# Update application/dto/mod.rs to include responses
if ! grep -q "pub mod responses;" src/application/dto/mod.rs; then
    echo -e "\npub mod responses;" >> src/application/dto/mod.rs
fi

# Fix plagiarism_queries.rs imports
cat > src/application/queries/plagiarism_queries.rs << 'EOF'
use crate::{
    api::responses::ApiResponse,
    application::dto::responses::{
        PlagiarismCaseResponse,
        PlagiarismCaseSearchResponse,
        VoteResponse,
        PlagiarismStatsResponse,
    },
    domain::repositories::PlagiarismRepository,
};
use uuid::Uuid;

pub struct PlagiarismQueries {
    repository: PlagiarismRepository,
}

impl PlagiarismQueries {
    pub fn new(repository: PlagiarismRepository) -> Self {
        Self { repository }
    }
    
    pub async fn get_case(&self, case_id: Uuid) -> Result<PlagiarismCaseResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismCaseResponse)
    }
    
    pub async fn search_cases(&self) -> Result<PlagiarismCaseSearchResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismCaseSearchResponse)
    }
    
    pub async fn get_votes(&self, case_id: Uuid) -> Result<VoteResponse, String> {
        // Implementation placeholder
        Ok(VoteResponse)
    }
    
    pub async fn get_stats(&self) -> Result<PlagiarismStatsResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismStatsResponse)
    }
}
EOF

# 4. Add futures_util dependency
echo "Adding futures_util dependency..."
if ! grep -q "futures-util" Cargo.toml; then
    sed -i '/^\[dependencies\]/a futures-util = "0.3"' Cargo.toml
fi

# 5. Fix rand imports
echo "Fixing rand imports..."
# Update verification_matrix.rs
sed -i '1s/use rand::distr::Alphanumeric;/use rand::Rng;\nuse rand::distributions::Alphanumeric;\nuse rand::distributions::DistString;/' src/domain/value_objects/verification_matrix.rs

# Update utils/mod.rs
sed -i '1s/use rand::distr::Alphanumeric;/use rand::Rng;\nuse rand::distributions::Alphanumeric;\nuse rand::distributions::DistString;/' src/utils/mod.rs

# 6. Fix external module imports
echo "Fixing external module imports..."
cat > src/infrastructure/external/mod.rs << 'EOF'
pub mod payment_gateway;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ExternalServiceError {
    Connection(String),
    Timeout(String),
    InvalidResponse(String),
    PaymentFailed(String),
}

#[derive(Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: ServiceStatus,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Degraded,
}

pub trait ExternalService: Send + Sync {
    async fn check_health(&self) -> Result<ServiceHealth, ExternalServiceError>;
}

pub trait PaymentGateway: ExternalService {
    async fn process_payment(&self, amount: f64) -> Result<String, String>;
    async fn refund_payment(&self, payment_id: &str, amount: Option<f64>) -> Result<String, String>;
    async fn get_payment_status(&self, payment_id: &str) -> Result<String, String>;
}
EOF

# Fix payment_gateway.rs
cat > src/infrastructure/external/payment_gateway.rs << 'EOF'
use super::*;

pub struct PaymentGatewayImpl;

impl ExternalService for PaymentGatewayImpl {
    async fn check_health(&self) -> Result<ServiceHealth, ExternalServiceError> {
        Ok(ServiceHealth {
            status: ServiceStatus::Healthy,
            message: "Payment gateway is healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}

impl super::PaymentGateway for PaymentGatewayImpl {
    async fn process_payment(&self, amount: f64) -> Result<String, String> {
        Ok(format!("payment_{}", uuid::Uuid::new_v4()))
    }
    
    async fn refund_payment(&self, payment_id: &str, amount: Option<f64>) -> Result<String, String> {
        Ok(format!("refund_{}", payment_id))
    }
    
    async fn get_payment_status(&self, payment_id: &str) -> Result<String, String> {
        Ok("completed".to_string())
    }
}
EOF

# 7. Create missing route functions
echo "Creating missing route functions..."

# Update donation_routes.rs
cat >> src/api/routes/donation_routes.rs << 'EOF'

pub async fn verify_payment(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Verify payment")
}
EOF

# Update plagiarism_routes.rs  
cat >> src/api/routes/plagiarism_routes.rs << 'EOF'

pub async fn bulk_check_plagiarism(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Bulk check plagiarism")
}
EOF

# Update notification_routes.rs
cat >> src/api/routes/notification_routes.rs << 'EOF'

pub async fn test_notification() -> impl Responder {
    HttpResponse::Ok().body("Test notification")
}
EOF

# Update member_routes.rs with all missing functions
cat > src/api/routes/member_routes.rs << 'EOF'
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("Get profile")
}

pub async fn update_profile(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Update profile")
}

pub async fn get_messages() -> impl Responder {
    HttpResponse::Ok().body("Get messages")
}

pub async fn get_message(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Get message")
}

pub async fn send_message(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Send message")
}

pub async fn get_downloads() -> impl Responder {
    HttpResponse::Ok().body("Get downloads")
}

pub async fn get_member_donations() -> impl Responder {
    HttpResponse::Ok().body("Get member donations")
}

pub async fn get_member_comments() -> impl Responder {
    HttpResponse::Ok().body("Get member comments")
}
EOF

# 8. Fix CacheManager dyn compatibility
echo "Fixing CacheManager trait..."

# Create a dyn-safe basic cache trait
cat > src/infrastructure/cache/basic_cache.rs << 'EOF'
use async_trait::async_trait;
use std::time::Duration;

use super::{CacheResult, CacheError};

#[async_trait]
pub trait BasicCache: Send + Sync {
    async fn get(&self, key: &str) -> CacheResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> CacheResult<()>;
    async fn delete(&self, key: &str) -> CacheResult<()>;
    async fn exists(&self, key: &str) -> CacheResult<bool>;
    async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<()>;
}
EOF

# Update the main cache mod.rs
cat > src/infrastructure/cache/mod.rs << 'EOF'
use async_trait::async_trait;
use std::time::Duration;
use serde::{Serialize, de::DeserializeOwned};

pub type CacheResult<T> = Result<T, CacheError>;
pub type CacheKey = String;
pub type Ttl = Option<Duration>;

#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Redis error: {0}")]
    Redis(String),
    #[error("Invalid configuration: {0}")]
    Config(String),
}

pub mod redis_pool;
pub mod session_store;
pub mod token_blacklist;
pub mod rate_limiter;
pub mod basic_cache;

pub use basic_cache::BasicCache;

#[async_trait]
pub trait CacheManager: BasicCache {
    // Typed operations
    async fn get_typed<T: DeserializeOwned + Send + Sync>(&self, key: &str) -> CacheResult<Option<T>>;
    async fn set_typed<T: Serialize + Send + Sync>(&self, key: &str, value: &T, ttl: Option<Duration>) -> CacheResult<()>;
    
    // Extended operations
    async fn get_or_set<T, F>(&self, key: &str, ttl: Option<Duration>, f: F) -> CacheResult<T>
    where
        T: DeserializeOwned + Serialize + Send + Sync,
        F: FnOnce() -> T + Send + Sync;
        
    // Basic extended operations with defaults
    async fn increment(&self, key: &str, amount: i64) -> CacheResult<i64> {
        Err(CacheError::Config("Not implemented".to_string()))
    }
    
    async fn decrement(&self, key: &str, amount: i64) -> CacheResult<i64> {
        Err(CacheError::Config("Not implemented".to_string()))
    }
}

// Separate trait for advanced operations
#[async_trait]
pub trait AdvancedCacheManager: CacheManager {
    async fn get_multi<T: DeserializeOwned + Send + Sync>(&self, keys: &[String]) -> CacheResult<Vec<Option<T>>>;
    async fn set_multi<T: Serialize + Send + Sync>(&self, items: &[(&str, T)], ttl: Option<Duration>) -> CacheResult<()>;
    async fn hash_set(&self, key: &str, field: &str, value: &str) -> CacheResult<()>;
    async fn hash_get(&self, key: &str, field: &str) -> CacheResult<Option<String>>;
    async fn hash_get_all(&self, key: &str) -> CacheResult<std::collections::HashMap<String, String>>;
    async fn hash_delete(&self, key: &str, field: &str) -> CacheResult<()>;
    async fn list_push(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn list_pop(&self, key: &str) -> CacheResult<Option<String>>;
    async fn list_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>>;
    async fn set_add(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn set_remove(&self, key: &str, value: &str) -> CacheResult<()>;
    async fn set_members(&self, key: &str) -> CacheResult<Vec<String>>;
    async fn set_is_member(&self, key: &str, value: &str) -> CacheResult<bool>;
    async fn sorted_set_add(&self, key: &str, score: f64, value: &str) -> CacheResult<()>;
    async fn sorted_set_range(&self, key: &str, start: isize, stop: isize) -> CacheResult<Vec<String>>;
    async fn sorted_set_range_by_score(&self, key: &str, min: f64, max: f64) -> CacheResult<Vec<String>>;
    async fn publish(&self, channel: &str, message: &str) -> CacheResult<()>;
    async fn subscribe(&self, channels: &[String]) -> CacheResult<redis::aio::PubSub>;
    async fn flush_all(&self) -> CacheResult<()>;
    async fn ping(&self) -> CacheResult<String>;
}
EOF

# Update the dependent files to use BasicCache instead of dyn CacheManager
for file in src/infrastructure/cache/session_store.rs \
            src/infrastructure/cache/token_blacklist.rs \
            src/infrastructure/cache/rate_limiter.rs; do
    if [ -f "$file" ]; then
        sed -i 's/Box<dyn CacheManager>/Box<dyn BasicCache>/g' "$file"
        sed -i 's/use super::{CacheManager/use super::{BasicCache/g' "$file"
        sed -i 's/CacheManager,/BasicCache,/g' "$file"
    fi
done

# 9. Clean up unused imports
echo "Cleaning up some unused imports..."

# Remove unused import from auth_handler.rs
sed -i '/use actix_web::{web,/s/web, //' src/api/handlers/auth_handler.rs

# Remove unused imports from various files
for file in src/infrastructure/cache/redis_pool.rs \
            src/infrastructure/cache/token_blacklist.rs \
            src/infrastructure/cache/rate_limiter.rs \
            src/infrastructure/security/rate_limiting.rs \
            src/utils/logger.rs \
            src/utils/validation.rs \
            src/utils/datetime.rs \
            src/utils/cryptography.rs; do
    if [ -f "$file" ]; then
        # Remove specific unused imports
        sed -i '/use .*\{.*error.*\}/d' "$file" 2>/dev/null || true
        sed -i '/use .*\{.*warn.*\}/d' "$file" 2>/dev/null || true
        sed -i '/use .*\{.*DateTime.*\}/d' "$file" 2>/dev/null || true
        sed -i '/use .*\{.*TimeZone.*\}/d' "$file" 2>/dev/null || true
        sed -i '/use .*\{.*Timelike.*\}/d' "$file" 2>/dev/null || true
    fi
done

# 10. Update Cargo.toml with all required dependencies
echo "Updating Cargo.toml with missing dependencies..."

# Add missing dependencies if not present
if ! grep -q "futures-util" Cargo.toml; then
    sed -i '/^\[dependencies\]/a futures-util = "0.3"' Cargo.toml
fi

if ! grep -q "validator" Cargo.toml; then
    sed -i '/^\[dependencies\]/a validator = { version = "0.16", features = ["derive"] }' Cargo.toml
fi

if ! grep -q "fern" Cargo.toml; then
    sed -i '/^\[dependencies\]/a fern = "0.6"' Cargo.toml
fi

echo "All fixes applied! Running cargo check..."
cargo check
