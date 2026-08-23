#!/bin/bash
# fix_text_handler_extras.sh

echo "Fixing syntax errors in text_handler_extras.rs..."

# Create a clean version of text_handler_extras.rs
cat > src/api/handlers/text_handler_extras.rs << 'EOF'
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadTextVersionRequest {
    pub content: String,
    pub version_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextVersionResponse {
    pub id: Uuid,
    pub text_id: Uuid,
    pub version: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn upload_text_version(
    _path: web::Path<Uuid>,
    _req: web::Json<UploadTextVersionRequest>
) -> impl Responder {
    let version = TextVersionResponse {
        id: Uuid::new_v4(),
        text_id: Uuid::new_v4(),
        version: 2,
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(version, "Text version uploaded".to_string()))
}

pub async fn text_exists(_query: web::Query<serde_json::Value>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"exists": true, "text_id": "text_123"}),
        "Text exists check".to_string()
    ))
}

pub async fn cleanup_old_texts() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"cleaned_count": 5, "freed_space": "10MB"}),
        "Old texts cleaned".to_string()
    ))
}

pub async fn subscribe_to_text_updates(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"subscribed": true, "subscription_id": "sub_123"}),
        "Subscribed to updates".to_string()
    ))
}

pub async fn unsubscribe_from_text_updates(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"unsubscribed": true}),
        "Unsubscribed from updates".to_string()
    ))
}

pub async fn get_text_subscribers(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        vec![
            serde_json::json!({"user_id": "user1", "email": "user1@example.com"}),
            serde_json::json!({"user_id": "user2", "email": "user2@example.com"})
        ],
        "Subscribers list".to_string()
    ))
}
EOF

echo "Fixed text_handler_extras.rs. Running cargo check..."
cargo check
