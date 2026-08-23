// backend/src/api/routes/auth_routes.rs
use axum::{
    Router,
    routing::{post, get},
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::sync::Arc;

use crate::application::services::AuthService;
use crate::api::handlers::auth_handler;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 128))]
    pub password: String,
    
    #[validate(length(max = 100))]
    pub organization: Option<String>,
    
    pub introduced_by: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 10))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmailRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 6, max = 6))]
    pub code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 10))]
    pub token: String,
    
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 8, max = 128))]
    pub old_password: String,
    
    #[validate(length(min = 8, max = 128))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub member_id: i32,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
}

pub fn routes(auth_service: Arc<dyn AuthService>) -> Router {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/logout", post(auth_handler::logout))
        .route("/refresh", post(auth_handler::refresh))
        .route("/verify-email", post(auth_handler::verify_email))
        .route("/request-password-reset", post(auth_handler::request_password_reset))
        .route("/reset-password", post(auth_handler::reset_password))
        .route("/change-password", post(auth_handler::change_password))
        .with_state(auth_service)
}
