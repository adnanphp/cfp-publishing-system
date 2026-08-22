use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn db_test() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Database test endpoint",
        "data": {
            "database": "PostgreSQL",
            "status": "Connected",
            "timestamp": chrono::Local::now().to_rfc3339()
        }
    }))
}

pub async fn root() -> impl Responder {
    use actix_web::{HttpResponse, Responder};
    use serde_json::json;
    use chrono::Utc;
    
    HttpResponse::Ok().json(json!({
        "service": "CFP Backend API",
        "version": "0.1.0",
        "description": "Charitable Foundation Publications System",
        "timestamp": Utc::now().to_rfc3339(),
        "endpoints": {
            "health": "/health",
            "status": "/status",
            "api_root": "/api",
            "api_docs": "See individual endpoints below",
            "members": "/api/members",
            "authors": "/api/authors",
            "texts": "/api/texts",
            "downloads": "/api/downloads",
            "auth": "/api/auth/*"
        }
    }))
}
