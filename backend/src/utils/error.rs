//! Comprehensive error handling for the CFP backend
//!
//! This module defines the application's error types, provides conversion
//! from various error sources, and implements proper error responses.

use std::fmt;
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;

/// Main application error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
    pub trace_id: Option<String>,
}

impl AppError {
    /// Create a new application error
    pub fn new(error_type: AppErrorType, message: String) -> Self {
        AppError {
            error_type,
            message,
            details: None,
            timestamp: chrono::Utc::now(),
            request_id: None,
            trace_id: None,
        }
    }
    
    /// Create a new application error with details
    pub fn new_with_details(
        error_type: AppErrorType,
        message: String,
        details: String,
    ) -> Self {
        AppError {
            error_type,
            message,
            details: Some(details),
            timestamp: chrono::Utc::now(),
            request_id: None,
            trace_id: None,
        }
    }
    
    /// Add request ID to error
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
    
    /// Add trace ID to error
    pub fn with_trace_id(mut self, trace_id: String) -> Self {
        self.trace_id = Some(trace_id);
        self
    }
    
    /// Add details to error
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::Validation => StatusCode::BAD_REQUEST,
            AppErrorType::Authentication => StatusCode::UNAUTHORIZED,
            AppErrorType::Authorization => StatusCode::FORBIDDEN,
            AppErrorType::NotFound => StatusCode::NOT_FOUND,
            AppErrorType::Conflict => StatusCode::CONFLICT,
            AppErrorType::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            AppErrorType::Database => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::ExternalService => StatusCode::BAD_GATEWAY,
            AppErrorType::Configuration => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Payment => StatusCode::PAYMENT_REQUIRED,
            AppErrorType::Timeout => StatusCode::REQUEST_TIMEOUT,
        }
    }
    
    /// Check if error is client-side (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status_code().is_client_error()
    }
    
    /// Check if error is server-side (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status_code().is_server_error()
    }
    
    /// Create a validation error
    pub fn validation(message: &str) -> Self {
        Self::new(AppErrorType::Validation, message.to_string())
    }
    
    /// Create an authentication error
    pub fn authentication(message: &str) -> Self {
        Self::new(AppErrorType::Authentication, message.to_string())
    }
    
    /// Create an authorization error
    pub fn authorization(message: &str) -> Self {
        Self::new(AppErrorType::Authorization, message.to_string())
    }
    
    /// Create a not found error
    pub fn not_found(message: &str) -> Self {
        Self::new(AppErrorType::NotFound, message.to_string())
    }
    
    /// Create a conflict error
    pub fn conflict(message: &str) -> Self {
        Self::new(AppErrorType::Conflict, message.to_string())
    }
    
    /// Create a rate limit error
    pub fn rate_limit(message: &str) -> Self {
        Self::new(AppErrorType::RateLimit, message.to_string())
    }
    
    /// Create a database error
    pub fn database(message: &str) -> Self {
        Self::new(AppErrorType::Database, message.to_string())
    }
    
    /// Create an external service error
    pub fn external_service(message: &str) -> Self {
        Self::new(AppErrorType::ExternalService, message.to_string())
    }
    
    /// Create a configuration error
    pub fn configuration(message: &str) -> Self {
        Self::new(AppErrorType::Configuration, message.to_string())
    }
    
    /// Create an internal error
    pub fn internal(message: &str) -> Self {
        Self::new(AppErrorType::Internal, message.to_string())
    }
    
    /// Create a payment error
    pub fn payment(message: &str) -> Self {
        Self::new(AppErrorType::Payment, message.to_string())
    }
    
    /// Create a timeout error
    pub fn timeout(message: &str) -> Self {
        Self::new(AppErrorType::Timeout, message.to_string())
    }
}

/// Types of application errors
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppErrorType {
    #[serde(rename = "validation_error")]
    Validation,
    
    #[serde(rename = "authentication_error")]
    Authentication,
    
    #[serde(rename = "authorization_error")]
    Authorization,
    
    #[serde(rename = "not_found_error")]
    NotFound,
    
    #[serde(rename = "conflict_error")]
    Conflict,
    
    #[serde(rename = "rate_limit_error")]
    RateLimit,
    
    #[serde(rename = "database_error")]
    Database,
    
    #[serde(rename = "external_service_error")]
    ExternalService,
    
    #[serde(rename = "configuration_error")]
    Configuration,
    
    #[serde(rename = "internal_error")]
    Internal,
    
    #[serde(rename = "payment_error")]
    Payment,
    
    #[serde(rename = "timeout_error")]
    Timeout,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppErrorType::Validation => write!(f, "validation_error"),
            AppErrorType::Authentication => write!(f, "authentication_error"),
            AppErrorType::Authorization => write!(f, "authorization_error"),
            AppErrorType::NotFound => write!(f, "not_found_error"),
            AppErrorType::Conflict => write!(f, "conflict_error"),
            AppErrorType::RateLimit => write!(f, "rate_limit_error"),
            AppErrorType::Database => write!(f, "database_error"),
            AppErrorType::ExternalService => write!(f, "external_service_error"),
            AppErrorType::Configuration => write!(f, "configuration_error"),
            AppErrorType::Internal => write!(f, "internal_error"),
            AppErrorType::Payment => write!(f, "payment_error"),
            AppErrorType::Timeout => write!(f, "timeout_error"),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
    
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse::from(self);
        
        HttpResponse::build(status_code)
            .insert_header(("Content-Type", "application/json"))
            .json(error_response)
    }
}

/// Error response for API clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
    pub trace_id: Option<String>,
    pub documentation_url: Option<String>,
}

/// Detailed error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: u16,
    pub error_type: String,
    pub message: String,
    pub details: Option<String>,
    pub fields: Option<Vec<FieldError>>,
}

/// Field-specific validation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub error: String,
    pub value: Option<serde_json::Value>,
}

impl From<&AppError> for ErrorResponse {
    fn from(error: &AppError) -> Self {
        ErrorResponse {
            error: ErrorInfo {
                code: error.status_code().as_u16(),
                error_type: error.error_type.to_string(),
                message: error.message.clone(),
                details: error.details.clone(),
                fields: None,
            },
            timestamp: error.timestamp,
            request_id: error.request_id.clone(),
            trace_id: error.trace_id.clone(),
            documentation_url: Some("https://cfp.example.com/docs/errors".to_string()),
        }
    }
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        ErrorResponse::from(&error)
    }
}

/// Extension trait for Result to add context
pub trait ResultExt<T, E> {
    fn context<C>(self, context: C) -> Result<T, AppError>
    where
        C: Into<String>;
    
    fn with_context<C, F>(self, context: F) -> Result<T, AppError>
    where
        C: Into<String>,
        F: FnOnce() -> C;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Into<AppError>,
{
    fn context<C>(self, context: C) -> Result<T, AppError>
    where
        C: Into<String>,
    {
        self.map_err(|e| {
            let mut app_error: AppError = e.into();
            app_error.message = format!("{}: {}", context.into(), app_error.message);
            app_error
        })
    }
    
    fn with_context<C, F>(self, context: F) -> Result<T, AppError>
    where
        C: Into<String>,
        F: FnOnce() -> C,
    {
        self.map_err(|e| {
            let mut app_error: AppError = e.into();
            app_error.message = format!("{}: {}", context().into(), app_error.message);
            app_error
        })
    }
}

/// Conversion from various error types to AppError

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::new(
            AppErrorType::Internal,
            format!("I/O error: {}", error),
        )
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::new(
            AppErrorType::Validation,
            format!("JSON serialization error: {}", error),
        )
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        AppError::new(
            AppErrorType::Validation,
            format!("UUID error: {}", error),
        )
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(error: chrono::ParseError) -> Self {
        AppError::new(
            AppErrorType::Validation,
            format!("Date/time parsing error: {}", error),
        )
    }
}

impl From<regex::Error> for AppError {
    fn from(error: regex::Error) -> Self {
        AppError::new(
            AppErrorType::Configuration,
            format!("Regex compilation error: {}", error),
        )
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        AppError::new(
            AppErrorType::Internal,
            format!("Cryptography error: {}", error),
        )
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        match error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::authentication("Token has expired")
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::authentication("Invalid token")
            }
            _ => AppError::new(
                AppErrorType::Authentication,
                format!("JWT error: {}", error),
            ),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let field_errors: Vec<FieldError> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| FieldError {
                    field: field.to_string(),
                    error: error.message.clone().unwrap_or_default(),
                    value: None,
                })
            })
            .collect();
        
        let mut app_error = AppError::validation("Validation failed");
        app_error.details = Some(format!("{} validation errors", field_errors.len()));
        
        // Create error response with field errors
        let mut error_response = ErrorResponse::from(&app_error);
        error_response.error.fields = Some(field_errors);
        
        app_error
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => {
                AppError::not_found("Resource not found")
            }
            diesel::result::Error::DatabaseError(kind, info) => {
                let message = format!("Database error: {}", info.message());
                match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        AppError::conflict(&message)
                    }
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        AppError::validation(&message)
                    }
                    _ => AppError::database(&message),
                }
            }
            _ => AppError::database(&format!("Database error: {}", error)),
        }
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(error: actix_web::error::Error) -> Self {
        AppError::new(
            AppErrorType::Internal,
            format!("Web framework error: {}", error),
        )
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            AppError::timeout("Request timeout")
        } else if error.is_connect() {
            AppError::external_service("Connection failed")
        } else {
            AppError::external_service(&format!("HTTP request error: {}", error))
        }
    }
}

impl From<lettre::transport::smtp::Error> for AppError {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        AppError::new(
            AppErrorType::ExternalService,
            format!("Email service error: {}", error),
        )
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::validation(&format!("Invalid integer: {}", error))
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(error: std::num::ParseFloatError) -> Self {
        AppError::validation(&format!("Invalid float: {}", error))
    }
}

/// Error builder for creating complex errors
pub struct ErrorBuilder {
    error: AppError,
}

impl ErrorBuilder {
    /// Start building a new error
    pub fn new(error_type: AppErrorType, message: &str) -> Self {
        ErrorBuilder {
            error: AppError::new(error_type, message.to_string()),
        }
    }
    
    /// Add details to the error
    pub fn details(mut self, details: &str) -> Self {
        self.error.details = Some(details.to_string());
        self
    }
    
    /// Add request ID to the error
    pub fn request_id(mut self, request_id: &str) -> Self {
        self.error.request_id = Some(request_id.to_string());
        self
    }
    
    /// Add trace ID to the error
    pub fn trace_id(mut self, trace_id: &str) -> Self {
        self.error.trace_id = Some(trace_id.to_string());
        self
    }
    
    /// Build the final error
    pub fn build(self) -> AppError {
        self.error
    }
}

/// Convenience functions for common error patterns

/// Create a validation error with field details
pub fn validation_error(field: &str, message: &str, value: Option<serde_json::Value>) -> AppError {
    let mut error = AppError::validation(&format!("Validation failed for field '{}'", field));
    
    let field_error = FieldError {
        field: field.to_string(),
        error: message.to_string(),
        value,
    };
    
    let error_response = ErrorResponse {
        error: ErrorInfo {
            code: StatusCode::BAD_REQUEST.as_u16(),
            error_type: "validation_error".to_string(),
            message: error.message.clone(),
            details: error.details.clone(),
            fields: Some(vec![field_error]),
        },
        timestamp: error.timestamp,
        request_id: error.request_id.clone(),
        trace_id: error.trace_id.clone(),
        documentation_url: Some("https://cfp.example.com/docs/errors".to_string()),
    };
    
    error
}

/// Create a not found error for a specific resource
pub fn not_found_error(resource_type: &str, resource_id: &str) -> AppError {
    AppError::not_found(&format!("{} '{}' not found", resource_type, resource_id))
}

/// Create a conflict error for duplicate resources
pub fn conflict_error(resource_type: &str, field: &str, value: &str) -> AppError {
    AppError::conflict(&format!("{} with {} '{}' already exists", resource_type, field, value))
}

/// Create an authorization error for insufficient permissions
pub fn authorization_error(required_permission: &str, resource: &str) -> AppError {
    AppError::authorization(&format!(
        "Insufficient permissions: '{}' required for '{}'",
        required_permission, resource
    ))
}

/// Error logging utilities

/// Log an error with appropriate level based on error type
pub fn log_error(error: &AppError) {
    if error.is_server_error() {
        log::error!("Server error: {:?}", error);
    } else if error.is_client_error() {
        match error.error_type {
            AppErrorType::Validation | AppErrorType::Authentication | AppErrorType::Authorization => {
                log::warn!("Client error: {:?}", error);
            }
            _ => {
                log::info!("Client error: {:?}", error);
            }
        }
    }
}

/// Log an error with context
pub fn log_error_with_context(error: &AppError, context: &str) {
    if error.is_server_error() {
        log::error!("{}: {:?}", context, error);
    } else if error.is_client_error() {
        log::warn!("{}: {:?}", context, error);
    } else {
        log::info!("{}: {:?}", context, error);
    }
}

/// Error recovery strategies

/// Retry a fallible operation with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    operation: F,
    max_retries: u32,
    initial_delay_ms: u64,
) -> Result<T, AppError>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, AppError>>>>,
{
    let mut retries = 0;
    let mut delay = initial_delay_ms;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                retries += 1;
                
                if retries >= max_retries {
                    log::error!("Operation failed after {} retries: {}", retries, err);
                    return Err(err);
                }
                
                // Only retry on certain error types
                match err.error_type {
                    AppErrorType::Database
                    | AppErrorType::ExternalService
                    | AppErrorType::Timeout
                    | AppErrorType::Internal => {
                        log::warn!(
                            "Operation failed (attempt {}/{}), retrying in {}ms: {}",
                            retries,
                            max_retries,
                            delay,
                            err
                        );
                        
                        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                        delay = delay * 2; // Exponential backoff
                    }
                    _ => {
                        // Don't retry on client errors
                        return Err(err);
                    }
                }
            }
        }
    }
}

/// Fallback to default value on error
pub fn fallback_on_error<T: Default>(result: Result<T, AppError>) -> Result<T, AppError> {
    match result {
        Ok(value) => Ok(value),
        Err(err) => {
            log::warn!("Using fallback value after error: {}", err);
            Ok(T::default())
        }
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_error_creation() {
        let error = AppError::validation("Test validation error");
        assert_eq!(error.error_type, AppErrorType::Validation);
        assert_eq!(error.message, "Test validation error");
        assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
        assert!(error.is_client_error());
        assert!(!error.is_server_error());
    }
    
    #[test]
    fn test_app_error_with_details() {
        let error = AppError::new_with_details(
            AppErrorType::Database,
            "Database connection failed".to_string(),
            "Connection timeout after 30 seconds".to_string(),
        );
        
        assert_eq!(error.error_type, AppErrorType::Database);
        assert_eq!(error.message, "Database connection failed");
        assert_eq!(error.details, Some("Connection timeout after 30 seconds".to_string()));
        assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        assert!(error.is_server_error());
    }
    
    #[test]
    fn test_error_response_from_app_error() {
        let app_error = AppError::not_found("User not found");
        let error_response = ErrorResponse::from(&app_error);
        
        assert_eq!(error_response.error.code, 404);
        assert_eq!(error_response.error.error_type, "not_found_error");
        assert_eq!(error_response.error.message, "User not found");
    }
    
    #[test]
    fn test_result_ext_context() {
        let result: Result<i32, AppError> = Err(AppError::validation("Invalid input"));
        
        let contextualized = result.context("While processing user input");
        
        match contextualized {
            Err(err) => {
                assert_eq!(err.error_type, AppErrorType::Validation);
                assert!(err.message.contains("While processing user input"));
                assert!(err.message.contains("Invalid input"));
            }
            Ok(_) => panic!("Expected error"),
        }
    }
    
    #[test]
    fn test_conversion_from_validation_errors() {
        use validator::Validate;
        
        #[derive(Debug, Validate)]
        struct TestStruct {
            #[validate(email)]
            email: String,
            
            #[validate(range(min = 18, max = 100))]
            age: i32,
        }
        
        let invalid = TestStruct {
            email: "invalid-email".to_string(),
            age: 15,
        };
        
        let validation_result = invalid.validate();
        assert!(validation_result.is_err());
        
        let app_error: AppError = validation_result.unwrap_err().into();
        assert_eq!(app_error.error_type, AppErrorType::Validation);
    }
    
    #[test]
    fn test_error_builder() {
        let error = ErrorBuilder::new(AppErrorType::Conflict, "Duplicate entry")
            .details("A record with this email already exists")
            .request_id("req-123")
            .trace_id("trace-456")
            .build();
        
        assert_eq!(error.error_type, AppErrorType::Conflict);
        assert_eq!(error.message, "Duplicate entry");
        assert_eq!(error.details, Some("A record with this email already exists".to_string()));
        assert_eq!(error.request_id, Some("req-123".to_string()));
        assert_eq!(error.trace_id, Some("trace-456".to_string()));
    }
}
