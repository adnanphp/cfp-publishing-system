pub mod payment_gateway;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ExternalServiceError {
    Connection(String),
    Timeout(String),
    InvalidResponse(String),
    PaymentFailed(String),
}

#[derive(Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: ServiceStatus,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Unhealthy,
    Degraded,
}

pub trait ExternalService: Send + Sync {
    async fn check_health(&self) -> Result<ServiceHealth, ExternalServiceError>;
}

pub trait PaymentGateway: ExternalService {
    async fn process_payment(&self, amount: f64) -> Result<String, String>;
    async fn refund_payment(&self, payment_id: &str, amount: Option<f64>) -> Result<String, String>;
    async fn get_payment_status(&self, payment_id: &str) -> Result<String, String>;
}
