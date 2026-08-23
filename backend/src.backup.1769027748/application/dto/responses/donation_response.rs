use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::DonationStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationResponse {
    pub donation_id: Uuid,
    pub member_id: Uuid,
    pub member_name: String,
    pub text_id: Uuid,
    pub text_title: String,
    pub charity_id: Uuid,
    pub charity_name: String,
    pub amount: f64,
    pub currency: String,
    pub status: DonationStatus,
    pub date: DateTime<Utc>,
    pub payment_method: String,
    pub transaction_id: Option<String>,
    pub charity_pct: i32,
    pub cfp_pct: i32,
    pub author_pct: i32,
    pub charity_amount: f64,
    pub cfp_amount: f64,
    pub author_amount: f64,
    pub anonymous: bool,
    pub message: Option<String>,
    pub rating: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationSummaryResponse {
    pub total_donations: u64,
    pub total_amount: f64,
    pub avg_donation_amount: f64,
    pub successful_donations: u64,
    pub failed_donations: u64,
    pub pending_donations: u64,
    pub refunded_donations: u64,
    pub top_donors: Vec<DonorSummary>,
    pub top_charities: Vec<CharitySummary>,
    pub top_texts: Vec<TextDonationSummary>,
    pub donations_by_day: Vec<DailyDonationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonorSummary {
    pub member_id: Uuid,
    pub member_name: String,
    pub total_donations: u64,
    pub total_amount: f64,
    pub last_donation_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharitySummary {
    pub charity_id: Uuid,
    pub charity_name: String,
    pub total_donations: u64,
    pub total_amount: f64,
    pub avg_donation_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDonationSummary {
    pub text_id: Uuid,
    pub text_title: String,
    pub total_donations: u64,
    pub total_amount: f64,
    pub author_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyDonationSummary {
    pub date: String,
    pub donation_count: u64,
    pub total_amount: f64,
    pub avg_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationSearchResponse {
    pub donations: Vec<DonationResponse>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub total_amount: f64,
    pub avg_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionResponse {
    pub donation_id: Uuid,
    pub charity_amount: f64,
    pub cfp_amount: f64,
    pub author_amount: f64,
    pub distribution_status: String,
    pub distribution_date: Option<DateTime<Utc>>,
    pub transaction_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayResponse {
    pub success: bool,
    pub transaction_id: String,
    pub gateway_response: serde_json::Value,
    pub redirect_url: Option<String>,
    pub payment_status: String,
    pub message: Option<String>,
}
