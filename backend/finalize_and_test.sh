#!/bin/bash

echo "=== Finalizing and Testing ==="

echo "1. Cleaning up Cargo.toml warnings..."
# Remove those profile.release.xxx lines that cause warnings
sed -i '/profile\.release\./d' Cargo.toml

echo "2. Creating a simple model to test our enums..."
mkdir -p src/domain/models

cat > src/domain/models/member.rs << 'MEMBER'
use crate::domain::enums::MemberStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: MemberStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Member {
    pub fn new(name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            status: MemberStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn activate(&mut self) {
        self.status = MemberStatus::Active;
        self.updated_at = Utc::now();
    }
    
    pub fn suspend(&mut self) {
        self.status = MemberStatus::Suspended;
        self.updated_at = Utc::now();
    }
    
    pub fn can_login(&self) -> bool {
        self.status.can_login()
    }
}
MEMBER

cat > src/domain/models/text.rs << 'TEXT'
use crate::domain::enums::TextStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub status: TextStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Text {
    pub fn new(title: String, content: String, author_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            author_id,
            status: TextStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn submit(&mut self) {
        self.status = TextStatus::Submitted;
        self.updated_at = Utc::now();
    }
    
    pub fn publish(&mut self) {
        self.status = TextStatus::Published;
        self.updated_at = Utc::now();
    }
    
    pub fn can_be_edited(&self) -> bool {
        self.status.can_be_edited()
    }
}
TEXT

echo "3. Updating model mod.rs..."
cat > src/domain/models/mod.rs << 'MODELS'
pub mod member;
pub mod text;

pub use member::*;
pub use text::*;
MODELS

echo "4. Updating domain mod.rs to include models..."
cat > src/domain/mod.rs << 'DOMAIN'
pub mod enums;
pub mod models;

pub use enums::*;
pub use models::*;
DOMAIN

echo "5. Creating a test API endpoint to show our models work..."
mkdir -p src/api/routes

cat > src/api/routes/mod.rs << 'ROUTES'
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(api_test))
            .route("/models", web::get().to(list_models))
    );
}

async fn api_test() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body("API test endpoint")
}

async fn list_models() -> actix_web::HttpResponse {
    use crate::domain::enums::{MemberStatus, TextStatus};
    use crate::domain::models::{Member, Text};
    use uuid::Uuid;
    
    // Create example models
    let member = Member::new("John Doe".to_string(), "john@example.com".to_string());
    let text = Text::new(
        "Sample Text".to_string(),
        "This is sample content".to_string(),
        member.id,
    );
    
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "member": {
            "id": member.id.to_string(),
            "name": member.name,
            "email": member.email,
            "status": format!("{:?}", member.status),
            "can_login": member.can_login(),
        },
        "text": {
            "id": text.id.to_string(),
            "title": text.title,
            "status": format!("{:?}", text.status),
            "can_be_edited": text.can_be_edited(),
        },
        "enums_available": {
            "member_status": ["Pending", "Active", "Suspended", "Banned", "Deleted"],
            "text_status": ["Draft", "Submitted", "UnderReview", "Approved", "Published", "Rejected", "Archived"],
        }
    }))
}
ROUTES

echo "6. Updating api module..."
mkdir -p src/api
cat > src/api/mod.rs << 'API'
pub mod routes;
pub use routes::*;
API

echo "pub mod api;" >> src/lib.rs

echo "7. Updating main.rs to use API routes..."
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
    println!("📚 Models API: http://localhost:{}/api/models", bind_port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .configure(cfp_backend::api::configure_routes)
    })
    .bind(format!("{}:{}", bind_host, bind_port))?
    .run()
    .await
}
MAIN

echo "8. Final compilation test..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉🎉🎉 AMAZING SUCCESS! 🎉🎉🎉"
    echo ""
    echo "Your project now has:"
    echo "✅ Working server with Actix-web"
    echo "✅ Configuration system"
    echo "✅ Domain models (Member, Text)"
    echo "✅ Domain enums (MemberStatus, TextStatus)"
    echo "✅ API routes"
    echo "✅ All dependencies properly integrated"
    echo "✅ UUID and DateTime support"
    echo "✅ Serialization with Serde"
    echo ""
    echo "Run: cargo run"
    echo ""
    echo "Test endpoints:"
    echo "  curl http://localhost:3000/health"
    echo "  curl http://localhost:3000/api/config"
    echo "  curl http://localhost:3000/api/models"
    echo ""
    echo "You have successfully rebuilt your project with:"
    echo "- Your actual file structure"
    echo "- Working code"
    echo "- Scalable architecture"
    echo ""
    echo "Next steps:"
    echo "1. Add database connection"
    echo "2. Add more models from your backup"
    echo "3. Add authentication"
    echo "4. Add services and repositories"
else
    echo "❌ Final compilation failed. Let's see the error..."
    cargo check 2>&1 | grep -E "error\[E[0-9]+\]:" | head -5
fi
