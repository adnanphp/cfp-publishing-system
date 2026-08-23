use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDonationRequest {
    #[validate(range(min = 1.0, message = "Minimum donation amount is $1.00"))]
    pub amount: f64,
    
    pub text_id: String,
    pub charity_id: String,
    pub currency: String,
    pub payment_method: String,
    
    #[validate(range(min = 60, max = 100, message = "Charity percentage must be between 60 and 100"))]
    pub charity_pct: i32,
    
    #[validate(range(min = 0, max = 40, message = "CFP percentage must be between 0 and 40"))]
    pub cfp_pct: i32,
    
    #[validate(range(min = 0, max = 40, message = "Author percentage must be between 0 and 40"))]
    pub author_pct: i32,
    
    pub anonymous: Option<bool>,
    pub message: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProcessDonationRequest {
    pub transaction_id: String,
    pub payment_status: String, // "succeeded", "failed", "pending"
    pub gateway_response: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RefundDonationRequest {
    pub reason: String,
    pub refund_amount: Option<f64>, // Partial refund if None, full refund
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchDonationsRequest {
    pub member_id: Option<String>,
    pub text_id: Option<String>,
    pub charity_id: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DonationSummaryRequest {
    pub period: String, // "day", "week", "month", "year", "custom"
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub group_by: Option<String>, // "charity", "text", "member", "date"
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDonationDistributionRequest {
    pub donation_id: String,
    
    #[validate(range(min = 60, max = 100))]
    pub charity_pct: i32,
    
    #[validate(range(min = 0, max = 40))]
    pub cfp_pct: i32,
    
    #[validate(range(min = 0, max = 40))]
    pub author_pct: i32,
}
