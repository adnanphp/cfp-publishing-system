use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

#[async_trait]
pub trait PlagiarismService: Send + Sync {
    async fn check_plagiarism(&self, text: String) -> Result<serde_json::Value, AppError>;
    async fn get_plagiarism_case(&self, case_id: Uuid) -> Result<serde_json::Value, AppError>;
}

pub struct PlagiarismServiceImpl;

impl PlagiarismServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PlagiarismService for PlagiarismServiceImpl {
    async fn check_plagiarism(&self, text: String) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({
            "similarity": 0.1,
            "original_text": text,
            "result": "No significant plagiarism detected"
        }))
    }

    async fn get_plagiarism_case(&self, case_id: Uuid) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({
            "id": case_id,
            "status": "resolved",
            "similarity_score": 0.15
        }))
    }
}
