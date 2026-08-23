use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send_notification(&self, user_id: Uuid, message: String) -> Result<Uuid, AppError>;
    async fn get_notifications(&self, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError>;
}

pub struct NotificationServiceImpl;

impl NotificationServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn send_notification(&self, user_id: Uuid, message: String) -> Result<Uuid, AppError> {
        println!("Sending notification to {}: {}", user_id, message);
        Ok(Uuid::new_v4())
    }

    async fn get_notifications(&self, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError> {
        Ok(vec![
            serde_json::json!({
                "id": Uuid::new_v4(),
                "user_id": user_id,
                "message": "Welcome to the platform",
                "read": false
            })
        ])
    }
}
