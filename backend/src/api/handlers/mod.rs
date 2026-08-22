pub mod auth_handler;
pub mod author_handler;
pub mod db_test_handler;
pub mod downloads_handler;
pub mod download_stats;
pub mod downloads_test;
pub mod member_handler;
pub mod text_handler;

// Re-export handler functions
pub use auth_handler::*;
pub use author_handler::*;
pub use db_test_handler::*;
pub use downloads_handler::*;
pub use download_stats::*;
pub use downloads_test::*;
pub use member_handler::*;
pub use text_handler::*;

// Define or re-export common handlers
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use chrono::Utc;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "cfp-backend",
        "timestamp": Utc::now().to_rfc3339()
    }))
}

pub async fn api_status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "operational",
        "database": "postgresql",
        "timestamp": Utc::now().to_rfc3339(),
        "endpoints": [
            "/health",
            "/api/health",
            "/api/members",
            "/api/authors",
            "/api/texts",
            "/api/downloads"
        ]
    }))
}

pub async fn root() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "service": "CFP Backend API",
        "version": "0.1.0",
        "description": "Charitable Foundation Publications System",
        "timestamp": Utc::now().to_rfc3339(),
        "documentation": "All endpoints are under /api path",
        "quick_links": {
            "health": "/health",
            "api_status": "/api/status",
            "members": "/api/members",
            "authors": "/api/authors",
            "texts": "/api/texts"
        }
    }))
}

// Make sure db_test is available
pub use db_test_handler::db_test;
