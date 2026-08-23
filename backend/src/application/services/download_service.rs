use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

#[async_trait]
pub trait DownloadService: Send + Sync {
    async fn record_download(&self, text_id: Uuid, user_id: Uuid) -> Result<Uuid, AppError>;
    async fn get_download_stats(&self, text_id: Uuid) -> Result<serde_json::Value, AppError>;
}

pub struct DownloadServiceImpl;

impl DownloadServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DownloadService for DownloadServiceImpl {
    async fn record_download(&self, text_id: Uuid, user_id: Uuid) -> Result<Uuid, AppError> {
        println!("Recording download of {} by {}", text_id, user_id);
        Ok(Uuid::new_v4())
    }

    async fn get_download_stats(&self, text_id: Uuid) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({
            "text_id": text_id,
            "total_downloads": 42,
            "unique_downloads": 30
        }))
    }
}
