#!/bin/bash

echo "=== Fixing Ownership Error in Main.rs ==="

echo "1. Creating fixed main.rs..."
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
    
    // Clone values needed outside the closure
    let bind_host = config.server.host.clone();
    let bind_port = config.server.port;
    
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
                    "database": config.database.url,
                    "status": "running"
                }))
            }))
    })
    .bind(format!("{}:{}", bind_host, bind_port))?
    .run()
    .await
}
MAIN

echo "2. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Ownership error fixed."
    echo ""
    echo "Now run: cargo run"
    echo "Then test: curl http://localhost:3000/api/config"
    echo ""
    echo "After confirming it works, we can add your actual files."
else
    echo "❌ Still errors. Let's go back to super simple..."
    
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
    echo ""
    echo "If this compiles, run: cargo run"
fi
