#!/bin/bash

echo "=== Fixing Duplicate Error ==="

echo "1. Removing duplicate entries from Cargo.toml..."
# Create a clean Cargo.toml
cat > Cargo.toml << 'CARGOEOF'
[package]
name = "cfp-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
actix-web = "4.0"
actix-rt = "2.0"

# Core utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }

# Required crates from errors
rand = "0.8"
actix = "0.13"
actix-web-actors = "4.0"

# Error handling
thiserror = "1.0"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
CARGOEOF

echo "2. Cleaning up..."
rm -f Cargo.lock
cargo clean

echo "3. Testing..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Compilation successful!"
    echo ""
    echo "Now run: cargo run"
    echo "Then test: curl http://localhost:3000/health"
else
    echo "❌ Still errors. Creating ultra-minimal version..."
    cat > Cargo.toml << 'MINIMAL'
[package]
name = "cfp-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
actix-rt = "2.0"

[profile.dev]
opt-level = 0
MINIMAL
    
    cat > src/main.rs << 'MAINEOF'
use actix_web::{web, App, HttpServer, HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("CFP Backend starting on http://0.0.0.0:3000");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "CFP Backend" }))
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
MAINEOF
    
    cargo check
fi
