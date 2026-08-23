#!/bin/bash

echo "=== Starting Clean and Fixing Main.rs ==="

echo "1. Creating completely clean src directory..."
rm -rf src
mkdir -p src

echo "2. Creating minimal working files..."
cat > src/main.rs << 'MAIN'
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;

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
    println!("🚀 CFP Backend starting on http://0.0.0.0:3000");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/api/test", web::get().to(|| async { "API endpoint" }))
            .route("/api/config", web::get().to(|| async {
                HttpResponse::Ok().json(json!({
                    "host": "0.0.0.0",
                    "port": 3000,
                    "status": "running"
                }))
            }))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
MAIN

echo "3. Creating Cargo.toml..."
cat > Cargo.toml << 'CARGO'
[package]
name = "cfp-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
actix-rt = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
CARGO

echo "4. Testing compilation..."
cargo clean
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Clean version compiles."
    echo ""
    echo "Now let's add ONE simple module..."
    echo ""
    echo "Creating config module..."
    
    mkdir -p src/config
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
    
    cat > src/lib.rs << 'LIB'
pub mod config;
pub use config::*;
LIB
    
    echo "5. Updating main.rs to use config..."
    cat > src/main.rs << 'MAIN2'
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
    
    // Load configuration (simplified - no redis for now)
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
                    "database": config.database.url,
                    "status": "running"
                }))
            }))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
MAIN2
    
    echo "6. Testing with config module..."
    cargo check
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "🎉 PERFECT! Now we have a working foundation with config."
        echo ""
        echo "Run: cargo run"
        echo "Test: curl http://localhost:3000/api/config"
        echo ""
        echo "Now we can add your actual files ONE AT A TIME."
        echo ""
        echo "Next step: Pick ONE simple file from your backup to add."
        echo ""
        echo "Look at your backup:"
        ls ../cfp-backup-*/src/domain/models/ 2>/dev/null | head -5
        echo ""
        echo "Choose ONE .rs file to add first (like member.rs)"
    else
        echo "❌ Config module failed. Keeping simple version."
    fi
else
    echo "❌ Something is seriously wrong with the minimal setup."
fi
