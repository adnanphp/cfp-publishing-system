use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

#[async_trait]
pub trait CommitteeService: Send + Sync {
    async fn create_committee(&self, name: String) -> Result<Uuid, AppError>;
    async fn get_committee(&self, committee_id: Uuid) -> Result<serde_json::Value, AppError>;
}

pub struct CommitteeServiceImpl;

impl CommitteeServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommitteeService for CommitteeServiceImpl {
    async fn create_committee(&self, name: String) -> Result<Uuid, AppError> {
        println!("Creating committee: {}", name);
        Ok(Uuid::new_v4())
    }

    async fn get_committee(&self, committee_id: Uuid) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({
            "id": committee_id,
            "name": "Sample Committee",
            "members": 5
        }))
    }
}
