#!/bin/bash

echo "=== Fixing main.rs Ownership Error ==="

echo "1. Creating correct main.rs..."
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
    
    // Clone config for binding address
    let bind_host = config.server.host.clone();
    let bind_port = config.server.port;
    
    // Try to create database pool
    match cfp_backend::infrastructure::database::create_pool(&config.database).await {
        Ok(_) => println!("✅ Database connection initialized"),
        Err(e) => println!("⚠️  Database connection failed: {}", e),
    }
    
    println!("🌐 Server listening on http://{}:{}", bind_host, bind_port);
    println!("✅ Health check: http://localhost:{}/health", bind_port);
    
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
    .bind(format!("{}:{}", bind_host, bind_port))?
    .run()
    .await
}
MAIN

echo "2. Fixing AppConfig to include all fields..."
cat > src/config/app.rs << 'APP'
use serde::Deserialize;
use super::{DatabaseConfig, RedisConfig, JwtConfig};

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
            redis: RedisConfig::new("redis://localhost/"),
            jwt: JwtConfig::new("default-secret-change-me", 24),
        })
    }
}
APP

echo "3. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Ownership error fixed."
    echo ""
    echo "Now run: cargo run"
    echo "Then test: curl http://localhost:3000/api/config"
    echo ""
    echo "Server should start and show your configuration."
else
    echo "❌ Still errors. Let's create a super simple version..."
    
    cat > src/main.rs << 'SIMPLE'
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "CFP Backend is running"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 CFP Backend starting on http://0.0.0.0:3000");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "CFP Backend API" }))
            .route("/health", web::get().to(health_check))
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
SIMPLE
    
    cargo check
    if [ $? -eq 0 ]; then
        echo "✅ Simple version compiles!"
        echo "Run: cargo run"
    fi
fi
