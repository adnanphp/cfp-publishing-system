use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub title: String,
    pub message: String,
    pub notification_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: String,
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub email_notifications: bool,
    pub push_notifications: bool,
}

pub async fn get_notifications() -> impl Responder {
    let notifications = vec![
        NotificationResponse {
            id: Uuid::new_v4(),
            title: "Welcome".to_string(),
            message: "Welcome to the platform".to_string(),
            notification_type: "system".to_string(),
            read: false,
            created_at: chrono::Utc::now(),
        },
        NotificationResponse {
            id: Uuid::new_v4(),
            title: "New comment".to_string(),
            message: "Someone commented on your text".to_string(),
            notification_type: "comment".to_string(),
            read: true,
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(notifications, "Notifications list".to_string()))
}

pub async fn get_unread_notifications() -> impl Responder {
    let notifications = vec![
        NotificationResponse {
            id: Uuid::new_v4(),
            title: "New message".to_string(),
            message: "You have a new message".to_string(),
            notification_type: "message".to_string(),
            read: false,
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(notifications, "Unread notifications".to_string()))
}

pub async fn mark_as_read(_path: web::Path<Uuid>) -> impl Responder {
    let notification = NotificationResponse {
        id: Uuid::new_v4(),
        title: "Marked as read".to_string(),
        message: "This notification was marked as read".to_string(),
        notification_type: "system".to_string(),
        read: true,
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(notification, "Notification marked as read".to_string()))
}

pub async fn mark_all_as_read() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"marked_count": 5}),
        "All notifications marked as read".to_string()
    ))
}

pub async fn create_notification(_req: web::Json<CreateNotificationRequest>) -> impl Responder {
    let notification = NotificationResponse {
        id: Uuid::new_v4(),
        title: "New notification".to_string(),
        message: "Notification created".to_string(),
        notification_type: "custom".to_string(),
        read: false,
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Created().json(ApiResponse::new(notification, "Notification created".to_string()))
}

pub async fn delete_notification(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"deleted": true}),
        "Notification deleted".to_string()
    ))
}

pub async fn get_notification_preferences() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "email_notifications": true,
            "push_notifications": false,
            "digest_frequency": "daily"
        }),
        "Notification preferences".to_string()
    ))
}

pub async fn update_notification_preferences(_req: web::Json<UpdatePreferencesRequest>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"updated": true}),
        "Preferences updated".to_string()
    ))
}

pub async fn get_notification_stats() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "total": 100,
            "unread": 15,
            "read": 85
        }),
        "Notification stats".to_string()
    ))
}
