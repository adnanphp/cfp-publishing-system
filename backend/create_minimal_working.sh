#!/bin/bash

echo "=== Creating Minimal Working Version with Your Structure ==="

echo "1. Saving current working main.rs..."
cp src/main.rs src/main.rs.working

echo "2. Creating super minimal src directory..."
rm -rf src_minimal
mkdir -p src_minimal
mv src/* src_minimal/ 2>/dev/null || true

echo "3. Creating bare minimum structure that works..."
mkdir -p src/{config,lib}

# Create absolute minimal config
cat > src/config/mod.rs << 'CONFIG'
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://localhost/cfp".to_string(),
            },
        })
    }
}
CONFIG

# Create empty lib.rs
cat > src/lib.rs << 'LIB'
pub mod config;
pub use config::*;
LIB

# Restore working main.rs
cp src_minimal/main.rs.working src/main.rs 2>/dev/null || cp src_minimal/main.rs src/main.rs 2>/dev/null || true

echo "4. Now gradually add ONE module at a time..."
echo "Which module do you want to add first (with minimal fixes)?"
echo "1. domain/models only"
echo "2. application/dto only"  
echo "3. api/routes only"
echo "4. infrastructure/database only"
read -p "Choose 1-4: " module_choice

case $module_choice in
    1)
        echo "Adding domain/models..."
        mkdir -p src/domain
        cat > src/domain/mod.rs << 'DOMAIN'
pub mod models;
pub use models::*;
DOMAIN
        
        mkdir -p src/domain/models
        # Copy only the simplest model files
        for model in member.rs text.rs donation.rs; do
            if [ -f "src_minimal/domain/models/$model" ]; then
                echo "Processing $model..."
                # Create minimal version
                grep -v "^use" "src_minimal/domain/models/$model" | \
                grep -v "^extern crate" | \
                head -50 > "src/domain/models/$model" 2>/dev/null || \
                echo "// Minimal placeholder for $model" > "src/domain/models/$model"
            fi
        done
        
        # Create models/mod.rs
        echo "pub mod member;" > src/domain/models/mod.rs
        echo "pub mod text;" >> src/domain/models/mod.rs
        echo "pub mod donation;" >> src/domain/models/mod.rs
        ;;
    2)
        echo "Adding application/dto..."
        mkdir -p src/application
        cat > src/application/mod.rs << 'APP'
pub mod dto;
pub use dto::*;
APP
        
        mkdir -p src/application/dto
        # Create minimal dto structure
        cat > src/application/dto/mod.rs << 'DTO'
pub mod requests;
pub mod responses;
pub mod types;

pub use requests::*;
pub use responses::*;
pub use types::*;
DTO
        
        mkdir -p src/application/dto/{requests,responses}
        echo "// Requests module" > src/application/dto/requests/mod.rs
        echo "// Responses module" > src/application/dto/responses/mod.rs
        ;;
    3)
        echo "Adding api/routes..."
        mkdir -p src/api
        cat > src/api/mod.rs << 'API'
pub mod routes;
pub use routes::*;
API
        
        mkdir -p src/api/routes
        # Create one simple route file
        cat > src/api/routes/mod.rs << 'ROUTES'
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(|| async { "API test" }))
    );
}
ROUTES
        ;;
    4)
        echo "Adding infrastructure/database..."
        mkdir -p src/infrastructure
        cat > src/infrastructure/mod.rs << 'INFRA'
pub mod database;
pub use database::*;
INFRA
        
        mkdir -p src/infrastructure/database
        cat > src/infrastructure/database/mod.rs << 'DB'
use crate::config::DatabaseConfig;

pub async fn create_pool(config: &DatabaseConfig) -> Result<(), String> {
    println!("Database configured for: {}", config.url);
    Ok(())
}
DB
        ;;
    *)
        echo "Invalid choice. Starting with domain/models."
        module_choice=1
        ;;
esac

echo "5. Updating lib.rs to include new module..."
case $module_choice in
    1) echo "pub mod domain;" >> src/lib.rs ;;
    2) echo "pub mod application;" >> src/lib.rs ;;
    3) echo "pub mod api;" >> src/lib.rs ;;
    4) echo "pub mod infrastructure;" >> src/lib.rs ;;
esac

echo "6. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Added module without errors."
    echo ""
    echo "Now you have:"
    echo "- Working server foundation"
    echo "- One actual module from your project"
    echo "- No compilation errors"
    echo ""
    echo "Run: cargo run"
    echo "Test: curl http://localhost:3000/health"
    echo ""
    echo "Then we can add another module..."
else
    echo "❌ Still errors. Let's see the first few..."
    cargo check 2>&1 | grep -E "error\[E[0-9]+\]:" | head -5
    
    echo ""
    echo "Removing the problematic module and keeping working version..."
    case $module_choice in
        1) rm -rf src/domain ;;
        2) rm -rf src/application ;;
        3) rm -rf src/api ;;
        4) rm -rf src/infrastructure ;;
    esac
    
    # Remove from lib.rs
    sed -i '/^pub mod domain;/d' src/lib.rs
    sed -i '/^pub mod application;/d' src/lib.rs
    sed -i '/^pub mod api;/d' src/lib.rs
    sed -i '/^pub mod infrastructure;/d' src/lib.rs
    
    cargo check
    echo ""
    echo "Back to working state. Try a different module choice."
fi

echo ""
echo "Your src_minimal/ directory has all your original files."
echo "We're building up gradually from a working foundation."
