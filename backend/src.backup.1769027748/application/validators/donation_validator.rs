use crate::{
    domain::value_objects::PercentageDistribution,
    utils::error::AppError,
};

pub struct DonationValidator;

impl DonationValidator {
    pub fn validate_distribution(distribution: &PercentageDistribution) -> Result<(), AppError> {
        // Charity percentage must be at least 60%
        if distribution.charity_pct < 60 {
            return Err(AppError::validation_error(
                format!("Charity percentage must be at least 60%, got {}", distribution.charity_pct)
            ));
        }
        
        // All percentages must sum to 100
        if distribution.charity_pct + distribution.cfp_pct + distribution.author_pct != 100 {
            return Err(AppError::validation_error(
                format!("Percentages must sum to 100, got charity:{}%, cfp:{}%, author:{}%", 
                    distribution.charity_pct, distribution.cfp_pct, distribution.author_pct)
            ));
        }
        
        // No percentage can be negative
        if distribution.charity_pct < 0 || distribution.cfp_pct < 0 || distribution.author_pct < 0 {
            return Err(AppError::validation_error("Percentages cannot be negative"));
        }
        
        Ok(())
    }
    
    pub fn validate_amount(amount: f64) -> Result<(), AppError> {
        // Minimum donation amount is $1.00
        if amount < 1.0 {
            return Err(AppError::validation_error(
                format!("Minimum donation amount is $1.00, got ${:.2}", amount)
            ));
        }
        
        // Maximum donation amount is $100,000.00
        if amount > 100_000.0 {
            return Err(AppError::validation_error(
                format!("Maximum donation amount is $100,000.00, got ${:.2}", amount)
            ));
        }
        
        // Amount should have at most 2 decimal places
        let rounded = (amount * 100.0).round() / 100.0;
        if (amount - rounded).abs() > 0.001 {
            return Err(AppError::validation_error("Amount must have at most 2 decimal places"));
        }
        
        Ok(())
    }
    
    pub fn validate_currency(currency: &str) -> Result<(), AppError> {
        let valid_currencies = ["USD", "EUR", "GBP", "CAD", "AUD", "JPY", "CHF"];
        
        if !valid_currencies.contains(&currency) {
            return Err(AppError::validation_error(
                format!("Unsupported currency: {}. Supported currencies: {:?}", currency, valid_currencies)
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_payment_method(payment_method: &str) -> Result<(), AppError> {
        let valid_methods = ["credit_card", "debit_card", "bank_transfer", "paypal", "stripe"];
        
        if !valid_methods.contains(&payment_method) {
            return Err(AppError::validation_error(
                format!("Unsupported payment method: {}. Supported methods: {:?}", payment_method, valid_methods)
            ));
        }
        
        Ok(())
    }
}
