use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

#[async_trait]
pub trait DonationService: Send + Sync {
    async fn create_donation(&self, amount: f64, text_id: Uuid, donor_id: Uuid) -> Result<Uuid, AppError>;
    async fn get_donation(&self, donation_id: Uuid) -> Result<serde_json::Value, AppError>;
}

pub struct DonationServiceImpl;

impl DonationServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DonationService for DonationServiceImpl {
    async fn create_donation(&self, _amount: f64, _text_id: Uuid, _donor_id: Uuid) -> Result<Uuid, AppError> {
        Ok(Uuid::new_v4())
    }

    async fn get_donation(&self, donation_id: Uuid) -> Result<serde_json::Value, AppError> {
        Ok(serde_json::json!({
            "id": donation_id,
            "amount": 100.0,
            "status": "completed"
        }))
    }
}
