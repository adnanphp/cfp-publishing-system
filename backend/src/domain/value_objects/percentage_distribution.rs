// backend/src/domain/value_objects/percentage_distribution.rs
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentageDistribution {
    pub charity_pct: u8,
    pub cfp_pct: u8,
    pub author_pct: u8,
}

#[derive(Debug, Error)]
pub enum DistributionError {
    #[error("Percentages must sum to 100")]
    InvalidSum,
    #[error("Charity percentage must be at least 60%")]
    CharityTooLow,
}

impl PercentageDistribution {
    pub fn new(charity_pct: u8, cfp_pct: u8, author_pct: u8) -> Result<Self, DistributionError> {
        let sum = charity_pct as u16 + cfp_pct as u16 + author_pct as u16;
        
        if sum != 100 {
            return Err(DistributionError::InvalidSum);
        }
        
        if charity_pct < 60 {
            return Err(DistributionError::CharityTooLow);
        }
        
        Ok(Self {
            charity_pct,
            cfp_pct,
            author_pct,
        })
    }
    
    pub fn default() -> Self {
        Self {
            charity_pct: 70,
            cfp_pct: 20,
            author_pct: 10,
        }
    }
    
    pub fn calculate_amounts(&self, total: f64) -> (f64, f64, f64) {
        (
            total * self.charity_pct as f64 / 100.0,
            total * self.cfp_pct as f64 / 100.0,
            total * self.author_pct as f64 / 100.0,
        )
    }
}
