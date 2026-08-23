pub mod auth_handler;
pub mod member_handler;
pub mod text_handler;
pub mod donation_handler;
pub mod plagiarism_handler;
pub mod committee_handler;
pub mod text_handler_extras;
pub mod notification_handler;
pub mod download_handler;

// Re-export handlers for easier access
pub use auth_handler::*;
pub use member_handler::*;
pub use text_handler::*;
pub use donation_handler::*;
pub use plagiarism_handler::*;
pub use committee_handler::*;
pub use text_handler_extras::*;
pub use notification_handler::*;
pub use download_handler::*;

use crate::api::responses::ApiResponse;
use actix_web::HttpResponse;

pub fn extract_user_id(req: &actix_web::HttpRequest) -> Result<uuid::Uuid, HttpResponse> {
    // Simplified implementation - would normally extract from JWT
    Ok(uuid::Uuid::new_v4())
}

pub fn require_role(req: &actix_web::HttpRequest, required_role: &str) -> Result<(), HttpResponse> {
    // Simplified implementation - would check user roles
    let _user_id = extract_user_id(req)?;
    // In real implementation, check if user has required_role
    Ok(())
}

pub fn require_authenticated(req: &actix_web::HttpRequest) -> Result<uuid::Uuid, HttpResponse> {
    extract_user_id(req)
}

// Helper functions for common response patterns
pub fn ok_response<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::new(data, "Success".to_string()))
}

pub fn created_response<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse::new(data, "Created".to_string()))
}

pub fn not_found_response(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ApiResponse::error(message.to_string()))
}

pub fn bad_request_response(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse::error(message.to_string()))
}

pub fn unauthorized_response(message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(ApiResponse::error(message.to_string()))
}

pub fn forbidden_response(message: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(ApiResponse::error(message.to_string()))
}

pub fn internal_error_response(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::error(message.to_string()))
}
