// backend/src/domain/models/donation.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::enums::DonationStatus;
use crate::domain::value_objects::PercentageDistribution;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Donation {
    pub donation_id: i32,
    
    pub member_id: i32,
    pub text_id: i32,
    pub charity_id: i32,
    
    #[validate(range(min = 1.0))]
    pub amount: f64,
    
    pub date: DateTime<Utc>,
    
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    
    #[validate(length(min = 1, max = 50))]
    pub payment_method: String,
    
    #[validate(length(min = 1, max = 100))]
    pub transaction_id: String,
    
    // Percentage distribution
    #[validate]
    pub distribution: PercentageDistribution,
    
    #[validate(email)]
    pub primary_email: String,
    
    // Calculated amounts
    pub charity_amount: f64,
    pub cfp_amount: f64,
    pub author_amount: f64,
    
    pub status: DonationStatus,
    pub failure_reason: Option<String>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Donation {
    pub fn new(
        member_id: i32,
        text_id: i32,
        charity_id: i32,
        amount: f64,
        currency: String,
        payment_method: String,
        transaction_id: String,
        distribution: PercentageDistribution,
    ) -> Result<Self, String> {
        let now = Utc::now();
        
        // Calculate amounts
        let (charity_amount, cfp_amount, author_amount) = distribution.calculate_amounts(amount);
        
        let donation = Self {
            donation_id: 0,
            member_id,
            text_id,
            charity_id,
            amount,
            date: now,
            currency,
            payment_method,
            transaction_id,
            distribution,
            charity_amount,
            cfp_amount,
            author_amount,
            status: DonationStatus::Pending,
            failure_reason: None,
            created_at: now,
            updated_at: now,
        };
        
        donation.validate()
            .map_err(|e| format!("Invalid donation data: {}", e))?;
        
        Ok(donation)
    }
    
    pub fn mark_processing(&mut self) {
        self.status = DonationStatus::Processing;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_completed(&mut self) {
        self.status = DonationStatus::Completed;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_failed(&mut self, reason: String) {
        self.status = DonationStatus::Failed;
        self.failure_reason = Some(reason);
        self.updated_at = Utc::now();
    }
    
    pub fn mark_refunded(&mut self) {
        self.status = DonationStatus::Refunded;
        self.updated_at = Utc::now();
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.status, DonationStatus::Completed)
    }
    
    pub fn can_be_refunded(&self) -> bool {
        matches!(self.status, DonationStatus::Completed)
            && (Utc::now() - self.date).num_days() < 30 // Within 30 days
    }
}
