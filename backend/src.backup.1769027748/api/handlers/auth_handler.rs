use actix_web::{HttpResponse, Responder};
use crate::api::responses::ApiResponse;

pub async fn login() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "token": "jwt_token",
            "refresh_token": "refresh_token"
        }),
        "Login successful".to_string()
    ))
}

pub async fn register() -> impl Responder {
    HttpResponse::Created().json(ApiResponse::new(
        serde_json::json!({
            "id": "user_id",
            "email": "user@example.com"
        }),
        "Registration successful".to_string()
    ))
}

pub async fn verify_email() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"verified": true}),
        "Email verified".to_string()
    ))
}

pub async fn request_password_reset() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"sent": true}),
        "Password reset email sent".to_string()
    ))
}

pub async fn reset_password() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"reset": true}),
        "Password reset successful".to_string()
    ))
}

pub async fn refresh_token() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "token": "new_jwt_token",
            "refresh_token": "new_refresh_token"
        }),
        "Token refreshed".to_string()
    ))
}

pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"logged_out": true}),
        "Logged out successfully".to_string()
    ))
}

pub async fn get_current_user() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "id": "user_id",
            "email": "user@example.com",
            "name": "Test User"
        }),
        "Current user data".to_string()
    ))
}
