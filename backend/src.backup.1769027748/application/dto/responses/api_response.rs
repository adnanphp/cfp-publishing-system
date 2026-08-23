use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
            status_code: 200,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn success_with_pagination(
        data: T,
        message: impl Into<String>,
        pagination: PaginationInfo,
    ) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
            status_code: 200,
            errors: None,
            pagination: Some(pagination),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn created(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.into(),
            status_code: 201,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message: impl Into<String>, status_code: u16) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: None,
            message: message.into(),
            status_code,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn validation_error(errors: Vec<ApiError>, message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: None,
            message: message.into(),
            status_code: 400,
            errors: Some(errors),
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: None,
            message: message.into(),
            status_code: 404,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: None,
            message: message.into(),
            status_code: 401,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            success: false,
            data: None,
            message: message.into(),
            status_code: 403,
            errors: None,
            pagination: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(json!(self));
        (status, body).into_response()
    }
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
            details: None,
        }
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
