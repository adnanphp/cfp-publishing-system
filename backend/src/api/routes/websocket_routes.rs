use validator::Validate;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::infrastructure::messaging::websocket_handler::WebSocketServer;

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    pub token: Option<String>,
    pub channels: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WebSocketConnectionResponse {
    pub connection_id: String,
    pub status: String,
    pub channels: Vec<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionStatus {
    pub total_connections: usize,
    pub active_users: Vec<i32>,
    pub channels: Vec<ChannelStatus>,
    pub server_uptime: String,
}

#[derive(Debug, Serialize)]
pub struct ChannelStatus {
    pub name: String,
    pub subscriber_count: usize,
    pub message_count: usize,
    pub last_message: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn websocket_handler(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    query: web::Query<WebSocketQuery>,
    ws_handler: web::Data<WebSocketServer>,
) -> Result<HttpResponse, actix_web::Error> {
    let token = query.token.clone().unwrap_or_default();
    
    // Log connection attempt
    log::info!("WebSocket connection attempt with token: {}", token);
    
    ws_handler.handle_connection(req, stream).await
}

pub async fn sse_handler(
    req: actix_web::HttpRequest,
    query: web::Query<WebSocketQuery>,
    sse_handler: web::Data<crate::infrastructure::messaging::sse_handler::SseHandler>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract user ID from token (in real implementation, validate JWT)
    let user_id = match extract_user_id_from_token(&query.token) {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    sse_handler.handle_connection(req, user_id).await
}

fn extract_user_id_from_token(token: &Option<String>) -> Option<i32> {
    // In a real implementation, validate JWT and extract user ID
    token.as_ref()?.parse::<i32>().ok()
}

pub async fn get_connection_status(
    ws_handler: web::Data<WebSocketServer>,
) -> impl Responder {
    // This would typically require admin privileges
    HttpResponse::Ok().json(serde_json::json!({
        "status": "operational",
        "connections": 0,
        "uptime": "0 days"
    }))
}

pub async fn send_message_via_websocket(
    request: web::Json<SendMessageRequest>,
    ws_handler: web::Data<WebSocketServer>,
) -> impl Responder {
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let message_request = request.into_inner();
    
    // In real implementation, send via WebSocket
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Message sent",
        "recipient_id": message_request.recipient_id
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct SendMessageRequest {
    #[validate(range(min = 1))]
    pub recipient_id: i32,
    
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    
    pub subject: Option<String>,
    
    pub priority: Option<String>,
}

pub async fn broadcast_to_channel(
    request: web::Json<BroadcastRequest>,
    ws_handler: web::Data<WebSocketServer>,
) -> impl Responder {
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let broadcast_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": broadcast_request.channel,
        "message": "Broadcast sent"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct BroadcastRequest {
    #[validate(length(min = 1, max = 100))]
    pub channel: String,
    
    #[validate(length(min = 1, max = 2000))]
    pub message: String,
    
    pub notification_type: Option<String>,
    
    pub metadata: Option<serde_json::Value>,
}

pub async fn get_active_channels() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "channels": [
            {
                "name": "notifications",
                "description": "System notifications",
                "subscriber_count": 0
            },
            {
                "name": "donations",
                "description": "Donation updates",
                "subscriber_count": 0
            }
        ]
    }))
}

pub async fn create_channel(
    request: web::Json<CreateChannelRequest>,
) -> impl Responder {
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let channel_request = request.into_inner();
    
    HttpResponse::Created().json(serde_json::json!({
        "channel": channel_request.name,
        "description": channel_request.description,
        "message": "Channel created"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChannelRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    
    pub is_private: Option<bool>,
    
    pub allowed_roles: Option<Vec<String>>,
    
    pub max_subscribers: Option<i32>,
}

pub async fn delete_channel(
    path: web::Path<String>,
) -> impl Responder {
    let channel_name = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": channel_name,
        "message": "Channel deleted"
    }))
}

pub async fn subscribe_to_channel(
    path: web::Path<String>,
) -> impl Responder {
    let channel_name = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": channel_name,
        "message": "Subscribed to channel"
    }))
}

pub async fn unsubscribe_from_channel(
    path: web::Path<String>,
) -> impl Responder {
    let channel_name = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": channel_name,
        "message": "Unsubscribed from channel"
    }))
}

pub async fn get_channel_subscribers(
    path: web::Path<String>,
) -> impl Responder {
    let channel_name = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": channel_name,
        "subscribers": [],
        "total": 0
    }))
}

pub async fn get_websocket_stats() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "stats": {
            "total_connections": 0,
            "active_connections": 0,
            "messages_sent": 0,
            "messages_received": 0,
            "connection_errors": 0,
            "avg_connection_duration": "0 minutes"
        }
    }))
}

pub async fn disconnect_user(
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "user_id": user_id,
        "message": "User disconnected from WebSocket"
    }))
}

pub async fn send_heartbeat() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now()
    }))
}

pub async fn get_message_history(
    query: web::Query<MessageHistoryQuery>,
) -> impl Responder {
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "messages": [],
        "query": query_params,
        "total": 0
    }))
}

#[derive(Debug, Deserialize)]
pub struct MessageHistoryQuery {
    pub channel: Option<String>,
    pub user_id: Option<i32>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<u32>,
}

pub async fn clear_channel_history(
    path: web::Path<String>,
) -> impl Responder {
    let channel_name = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "channel": channel_name,
        "message": "Channel history cleared",
        "cleared_count": 0
    }))
}

pub async fn get_connection_info(
    query: web::Query<ConnectionInfoQuery>,
) -> impl Responder {
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "connections": [],
        "query": query_params
    }))
}

#[derive(Debug, Deserialize)]
pub struct ConnectionInfoQuery {
    pub user_id: Option<i32>,
    pub ip_address: Option<String>,
    pub connected_since: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<u32>,
}
