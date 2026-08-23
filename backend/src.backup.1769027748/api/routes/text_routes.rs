use actix_web::{web, HttpResponse, Responder};
use crate::api::handlers::text_handler_extras;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::api::handlers::text_handler;

#[derive(Debug, Deserialize, Validate)]
pub struct SearchTextsQuery {
    #[validate(length(min = 1, max = 100))]
    pub query: Option<String>,
    
    pub topic: Option<String>,
    
    #[validate(length(min = 1, max = 50))]
    pub author: Option<String>,
    
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,
    
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTextRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(min = 10))]
    pub abstract_text: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub topic: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    
    pub content: String,
    
    pub author_orcid: Option<String>,
    
    pub version_notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTextRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    
    #[validate(length(min = 10))]
    pub abstract_text: Option<String>,
    
    #[validate(length(min = 1, max = 100))]
    pub topic: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    
    pub content: Option<String>,
    
    pub version_notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DownloadTextRequest {
    pub agree_to_terms: bool,
}

#[derive(Debug, Serialize)]
pub struct TextResponse {
    pub text_id: i32,
    pub title: String,
    pub abstract_text: Option<String>,
    pub topic: Option<String>,
    pub author_orcid: Option<String>,
    pub author_name: Option<String>,
    pub upload_date: chrono::NaiveDate,
    pub status: String,
    pub download_count: i32,
    pub total_donations: f64,
    pub avg_rating: Option<f64>,
    pub keywords: Vec<String>,
    pub version: i32,
}

#[derive(Debug, Serialize)]
pub struct TextListResponse {
    pub texts: Vec<TextResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct DownloadResponse {
    pub download_id: i32,
    pub text_id: i32,
    pub download_date: chrono::DateTime<chrono::Utc>,
    pub download_url: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub async fn search_texts(
    query: web::Query<SearchTextsQuery>,
) -> impl Responder {
    match text_handler::search_texts(query.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            log::error!("Error searching texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to search texts"
            }))
        }
    }
}

pub async fn upload_text_version(
    path: web::Path<i32>,
    request: web::Json<CreateTextRequest>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    match text_handler_extras::upload_text_version(text_id, request.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => {
            log::error!("Error uploading text version: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to upload text version"
            }))
        }
    }
}

pub async fn get_text_analytics(
    path: web::Path<i32>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    match text_handler::get_text_analytics(text_id).await {
        Ok(analytics) => HttpResponse::Ok().json(analytics),
        Err(e) => {
            log::error!("Error getting text analytics: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get text analytics"
            }))
        }
    }
}

pub async fn export_text(
    path: web::Path<i32>,
    query: web::Query<ExportFormatQuery>,
) -> impl Responder {
    let text_id = path.into_inner();
    let format = query.into_inner().format;
    
    match text_handler::export_text(text_id, format).await {
        Ok((content, filename, content_type)) => {
            HttpResponse::Ok()
                .content_type(content_type)
                .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
                .body(content)
        }
        Err(e) => {
            log::error!("Error exporting text: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to export text"
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExportFormatQuery {
    pub format: String,
}

pub async fn batch_upload_texts(
    request: web::Json<BatchUploadRequest>,
) -> impl Responder {
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    match text_handler::batch_upload_texts(request.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => {
            log::error!("Error batch uploading texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to batch upload texts"
            }))
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct BatchUploadRequest {
    pub texts: Vec<CreateTextRequest>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn get_popular_texts() -> impl Responder {
    match text_handler::get_popular_texts().await {
        Ok(texts) => HttpResponse::Ok().json(texts),
        Err(e) => {
            log::error!("Error getting popular texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get popular texts"
            }))
        }
    }
}

pub async fn get_recent_texts() -> impl Responder {
    match text_handler::get_recent_texts().await {
        Ok(texts) => HttpResponse::Ok().json(texts),
        Err(e) => {
            log::error!("Error getting recent texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get recent texts"
            }))
        }
    }
}

pub async fn get_author_texts(
    path: web::Path<String>,
) -> impl Responder {
    let orcid = path.into_inner();
    
    match text_handler::get_author_texts(&orcid).await {
        Ok(texts) => HttpResponse::Ok().json(texts),
        Err(e) => {
            log::error!("Error getting author texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get author texts"
            }))
        }
    }
}

pub async fn text_exists(
    query: web::Query<TextExistsQuery>,
) -> impl Responder {
    match text_handler_extras::text_exists(query.into_inner()).await {
        Ok(exists) => HttpResponse::Ok().json(serde_json::json!({ "exists": exists })),
        Err(e) => {
            log::error!("Error checking text existence: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check text existence"
            }))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TextExistsQuery {
    pub title: String,
    pub author_orcid: Option<String>,
}

pub async fn cleanup_old_texts() -> impl Responder {
    match text_handler_extras::cleanup_old_texts().await {
        Ok(count) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Cleanup completed",
            "texts_removed": count
        })),
        Err(e) => {
            log::error!("Error cleaning up old texts: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to cleanup old texts"
            }))
        }
    }
}

// WebSocket endpoints for real-time text updates
pub async fn subscribe_to_text_updates(
    path: web::Path<i32>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    match text_handler_extras::subscribe_to_text_updates(text_id).await {
        Ok(subscription) => HttpResponse::Ok().json(subscription),
        Err(e) => {
            log::error!("Error subscribing to text updates: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to subscribe to text updates"
            }))
        }
    }
}

pub async fn unsubscribe_from_text_updates(
    path: web::Path<i32>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    match text_handler_extras::unsubscribe_from_text_updates(text_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Unsubscribed successfully"
        })),
        Err(e) => {
            log::error!("Error unsubscribing from text updates: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to unsubscribe from text updates"
            }))
        }
    }
}

pub async fn get_text_subscribers(
    path: web::Path<i32>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    match text_handler_extras::get_text_subscribers(text_id).await {
        Ok(subscribers) => HttpResponse::Ok().json(subscribers),
        Err(e) => {
            log::error!("Error getting text subscribers: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get text subscribers"
            }))
        }
    }
}
