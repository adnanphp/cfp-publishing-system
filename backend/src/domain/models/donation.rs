use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Donation {
    pub donation_id: u32,
    pub member_id: u32,
    pub text_id: u32,
    pub charity_id: u32,
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub currency: String,
    pub payment_method: Option<String>,
    pub transaction_id: Option<String>,
    pub charity_pct: u32,
    pub cfp_pct: u32,
    pub author_pct: u32,
}

impl Donation {
    pub fn new(
        member_id: u32,
        text_id: u32,
        charity_id: u32,
        amount: f64,
    ) -> Self {
        Self {
            donation_id: 0, // Will be set by database
            member_id,
            text_id,
            charity_id,
            amount,
            date: Utc::now(),
            currency: "USD".to_string(),
            payment_method: None,
            transaction_id: None,
            charity_pct: 60,
            cfp_pct: 20,
            author_pct: 20,
        }
    }
    
    pub fn total_percentage(&self) -> u32 {
        self.charity_pct + self.cfp_pct + self.author_pct
    }
    
    pub fn charity_amount(&self) -> f64 {
        self.amount * (self.charity_pct as f64 / 100.0)
    }
    
    pub fn cfp_amount(&self) -> f64 {
        self.amount * (self.cfp_pct as f64 / 100.0)
    }
    
    pub fn author_amount(&self) -> f64 {
        self.amount * (self.author_pct as f64 / 100.0)
    }
}
