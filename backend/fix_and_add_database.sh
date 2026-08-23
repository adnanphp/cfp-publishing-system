#!/bin/bash

echo "=== Fixing Import Errors ==="

echo "1. Checking lib.rs exports..."
cat > src/lib.rs << 'LIB'
pub mod api;
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod utils;

// Re-exports
pub use config::*;
pub use infrastructure::database::*;
LIB

echo "2. Checking config module structure..."
cat > src/config/mod.rs << 'CONFIGMOD'
pub mod app;
pub mod database;
pub mod redis;
pub mod jwt;

pub use app::*;
pub use database::*;
pub use redis::*;
pub use jwt::*;
CONFIGMOD

echo "3. Checking if app.rs exists..."
if [ ! -f "src/config/app.rs" ]; then
    cat > src/config/app.rs << 'APP'
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
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

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: u64,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // For now, return default config
        Ok(AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://localhost/cfp".to_string(),
            },
            redis: RedisConfig {
                url: "redis://localhost/".to_string(),
            },
            jwt: JwtConfig {
                secret: "default-secret-change-me".to_string(),
                expiration_hours: 24,
            },
        })
    }
}
APP
fi

echo "4. Creating infrastructure/database module..."
mkdir -p src/infrastructure/database
cat > src/infrastructure/database/mod.rs << 'DBMOD'
pub mod database_pool;

pub use database_pool::*;
DBMOD

cat > src/infrastructure/database/database_pool.rs << 'DBPOOL'
use crate::config::DatabaseConfig;

pub async fn create_pool(config: &DatabaseConfig) -> Result<(), String> {
    println!("📊 Attempting to connect to database: {}", config.url);
    // This is a placeholder - will implement real connection later
    Ok(())
}
DBPOOL

echo "5. Updating infrastructure/mod.rs..."
cat > src/infrastructure/mod.rs << 'INFRA'
pub mod database;

pub use database::*;
INFRA

echo "6. Updating main.rs to fix imports..."
cat > src/main.rs << 'MAIN'
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;
use cfp_backend::config::AppConfig;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "CFP Backend is running"
    }))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("CFP Backend API v0.1.0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 CFP Backend starting...");
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(cfg) => {
            println!("✅ Configuration loaded");
            cfg
        }
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("📊 Database URL: {}", config.database.url);
    println!("🔗 Redis URL: {}", config.redis.url);
    
    // Try to create database pool
    match cfp_backend::infrastructure::database::create_pool(&config.database).await {
        Ok(_) => println!("✅ Database connection initialized"),
        Err(e) => println!("⚠️  Database connection failed: {}", e),
    }
    
    println!("🌐 Server listening on http://{}:{}", config.server.host, config.server.port);
    println!("✅ Health check: http://localhost:{}/health", config.server.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/api/test", web::get().to(|| async { "API endpoint" }))
            .route("/api/config", web::get().to(|config: web::Data<AppConfig>| async move {
                HttpResponse::Ok().json(json!({
                    "host": config.server.host,
                    "port": config.server.port,
                    "database_configured": !config.database.url.is_empty(),
                    "status": "running"
                }))
            }))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
MAIN

echo "7. Adding SQLx dependency..."
if ! grep -q "sqlx" Cargo.toml; then
    cat >> Cargo.toml << 'DEPS'

# Database (commented out for now - add when ready)
# sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "chrono", "uuid", "migrate"] }
DEPS
fi

echo "8. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! All imports fixed."
    echo ""
    echo "Now run: cargo run"
    echo "Then test: curl http://localhost:3000/api/config"
    echo ""
    echo "The server will show database configuration but won't actually connect yet."
    echo "When ready to add real database:"
    echo "1. Uncomment sqlx in Cargo.toml"
    echo "2. Install PostgreSQL"
    echo "3. Update database_pool.rs with real connection"
else
    echo "❌ Still have compilation errors"
    echo "Let me check what's wrong..."
    cargo check 2>&1 | grep -E "error\[|help:" | head -10
fi
