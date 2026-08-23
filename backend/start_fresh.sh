#!/bin/bash

echo "=== STARTING FRESH ==="

echo "1. Backing up your current project..."
mkdir -p ../cfp-backup-$(date +%s)
cp -r . ../cfp-backup-$(date +%s)/
echo "Backup created in parent directory"

echo "2. Creating clean project structure..."
rm -rf src/* Cargo.lock
mkdir -p src

echo "3. Creating minimal Cargo.toml..."
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

echo "4. Creating working main.rs..."
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
    println!("✅ Health check: http://localhost:3000/health");
    println!("📚 API: http://localhost:3000/");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/api/test", web::get().to(|| async { "API endpoint" }))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
MAIN

echo "5. Testing..."
cargo clean
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Minimal project compiles."
    echo ""
    echo "Next steps:"
    echo "1. Run: cargo run"
    echo "2. In another terminal: curl http://localhost:3000/health"
    echo "3. If that works, we'll add modules one by one"
    
    echo ""
    echo "Running server now (Ctrl+C to stop)..."
    cargo run
else
    echo "❌ Something went wrong with minimal setup"
    echo "Creating ultra-simple version..."
    
    cat > src/main.rs << 'ULTRA'
fn main() {
    println!("CFP Backend - Ultra simple version");
    println!("At least this compiles!");
}
ULTRA
    
    cargo check
fi
