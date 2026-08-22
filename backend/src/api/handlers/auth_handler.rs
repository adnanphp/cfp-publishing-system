use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn api_status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "CFP API is running",
        "version": "1.0.0",
        "endpoints": [
            "/health",
            "/api/status",
            "/api/members",
            "/api/authors",
            "/api/texts",
            "/api/db-test",
            "/api/auth/login",
            "/api/auth/register",
            "/api/auth/logout"
        ]
    }))
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "status": "healthy",
            "service": "CFP Backend",
            "timestamp": chrono::Local::now().to_rfc3339(),
            "database": "connected",
            "texts_count": 8,
            "authors_count": 3
        }))
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Login successful (placeholder)",
        "timestamp": chrono::Local::now().to_rfc3339()
    }))
}

pub async fn register() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Registration successful (placeholder)",
        "timestamp": chrono::Local::now().to_rfc3339()
    }))
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Logout successful (placeholder)",
        "timestamp": chrono::Local::now().to_rfc3339()
    }))
}
