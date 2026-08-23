use super::*;

pub struct PaymentGatewayImpl;

impl ExternalService for PaymentGatewayImpl {
    async fn check_health(&self) -> Result<ServiceHealth, ExternalServiceError> {
        Ok(ServiceHealth {
            status: ServiceStatus::Healthy,
            message: "Payment gateway is healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }
}

impl super::PaymentGateway for PaymentGatewayImpl {
    async fn process_payment(&self, amount: f64) -> Result<String, String> {
        Ok(format!("payment_{}", uuid::Uuid::new_v4()))
    }
    
    async fn refund_payment(&self, payment_id: &str, amount: Option<f64>) -> Result<String, String> {
        Ok(format!("refund_{}", payment_id))
    }
    
    async fn get_payment_status(&self, payment_id: &str) -> Result<String, String> {
        Ok("completed".to_string())
    }
}
