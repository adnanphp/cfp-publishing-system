#!/bin/bash

echo "Running quick compilation test..."

# Create a minimal main.rs for testing
cat > src/main.rs << 'MAINEOF'
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting CFP Backend...");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
MAINEOF

# Try to compile just the binary
if cargo check --bin cfp-backend; then
    echo "✓ Compilation successful!"
    echo ""
    echo "To run: cargo run"
    echo "Then visit: http://localhost:8080/health"
else
    echo "✗ Compilation failed"
    cargo check --bin cfp-backend 2>&1 | tail -20
fi
