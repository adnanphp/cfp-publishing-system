//! Validation utilities for the CFP system
//! Provides validation for various data types and business rules

use chrono::{DateTime, NaiveDate, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref ORCID_REGEX: Regex = Regex::new(r"^\d{4}-\d{4}-\d{4}-\d{3}[0-9X]$").unwrap();
    static ref PHONE_REGEX: Regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    static ref POSTAL_CODE_REGEX: Regex = Regex::new(r"^[A-Z0-9\s-]{3,10}$").unwrap();
    static ref PASSWORD_REGEX: Regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap();
}

/// Validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorType {
    InvalidEmail,
    InvalidPassword,
    InvalidPhone,
    InvalidOrcid,
    InvalidPostalCode,
    InvalidDate,
    InvalidAmount,
    InvalidPercentage,
    InvalidRating,
    InvalidVersion,
    TooManyKeywords,
    ContentTooShort,
    DuplicateValue,
    BusinessRuleViolation,
    RequiredField,
}

impl ValidationErrorType {
    pub fn message(&self) -> String {
        match self {
            Self::InvalidEmail => "Invalid email format".to_string(),
            Self::InvalidPassword => "Password must be at least 8 characters with uppercase, lowercase, number, and special character".to_string(),
            Self::InvalidPhone => "Invalid phone number format".to_string(),
            Self::InvalidOrcid => "Invalid ORCID format (XXXX-XXXX-XXXX-XXXX)".to_string(),
            Self::InvalidPostalCode => "Invalid postal code format".to_string(),
            Self::InvalidDate => "Invalid date".to_string(),
            Self::InvalidAmount => "Amount must be positive".to_string(),
            Self::InvalidPercentage => "Percentage must be between 0 and 100".to_string(),
            Self::InvalidRating => "Rating must be between 1 and 5".to_string(),
            Self::InvalidVersion => "Version must be at least 1".to_string(),
            Self::TooManyKeywords => "Maximum 10 keywords allowed".to_string(),
            Self::ContentTooShort => "Content must be at least 10 characters".to_string(),
            Self::DuplicateValue => "Duplicate value not allowed".to_string(),
            Self::BusinessRuleViolation => "Business rule violation".to_string(),
            Self::RequiredField => "This field is required".to_string(),
        }
    }
}

/// Validates an email address
pub fn validate_email(email: &str) -> Result<(), ValidationErrorType> {
    if EMAIL_REGEX.is_match(email) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidEmail)
    }
}

/// Validates a password against security requirements
pub fn validate_password(password: &str) -> Result<(), ValidationErrorType> {
    if PASSWORD_REGEX.is_match(password) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidPassword)
    }
}

/// Validates an ORCID identifier
pub fn validate_orcid(orcid: &str) -> Result<(), ValidationErrorType> {
    if ORCID_REGEX.is_match(orcid) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidOrcid)
    }
}

/// Validates a phone number (E.164 format)
pub fn validate_phone(phone: &str) -> Result<(), ValidationErrorType> {
    if PHONE_REGEX.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidPhone)
    }
}

/// Validates a postal code
pub fn validate_postal_code(postal_code: &str) -> Result<(), ValidationErrorType> {
    if POSTAL_CODE_REGEX.is_match(postal_code) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidPostalCode)
    }
}

/// Validates a date is not in the future
pub fn validate_date_not_future(date: &NaiveDate) -> Result<(), ValidationErrorType> {
    let today = Utc::now().date_naive();
    if date <= &today {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidDate)
    }
}

/// Validates donation percentages (charity ≥ 60%, total = 100%)
pub fn validate_donation_percentages(
    charity_pct: i32,
    cfp_pct: i32,
    author_pct: i32,
) -> Result<(), ValidationErrorType> {
    if charity_pct < 60 {
        return Err(ValidationErrorType::BusinessRuleViolation);
    }
    
    if charity_pct + cfp_pct + author_pct != 100 {
        return Err(ValidationErrorType::InvalidPercentage);
    }
    
    Ok(())
}

/// Validates a rating (1-5)
pub fn validate_rating(rating: i32) -> Result<(), ValidationErrorType> {
    if (1..=5).contains(&rating) {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidRating)
    }
}

/// Validates text version
pub fn validate_version(version: i32) -> Result<(), ValidationErrorType> {
    if version >= 1 {
        Ok(())
    } else {
        Err(ValidationErrorType::InvalidVersion)
    }
}

/// Validates keywords count (max 10)
pub fn validate_keywords(keywords: &[String]) -> Result<(), ValidationErrorType> {
    if keywords.len() <= 10 {
        Ok(())
    } else {
        Err(ValidationErrorType::TooManyKeywords)
    }
}

/// Validates comment content length (min 10 chars)
pub fn validate_comment_content(content: &str) -> Result<(), ValidationErrorType> {
    if content.trim().len() >= 10 {
        Ok(())
    } else {
        Err(ValidationErrorType::ContentTooShort)
    }
}

/// Validates minimum donation amount ($1.00)
pub fn validate_donation_amount(amount: f64) -> Result<(), ValidationErrorType> {
    if amount >= 1.0 {
        Ok(())
    } else {
        Err(ValidationErrorType::BusinessRuleViolation)
    }
}

/// Validates member status based on business rules
pub fn validate_member_status(
    is_donor: bool,
    violations_count: i32,
) -> Result<(), ValidationErrorType> {
    if violations_count >= 3 {
        return Err(ValidationErrorType::BusinessRuleViolation);
    }
    Ok(())
}

/// Validates multivalued fields for duplicates
pub fn validate_no_duplicates<T: Eq + std::hash::Hash>(values: &[T]) -> Result<(), ValidationErrorType> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(ValidationErrorType::DuplicateValue);
        }
    }
    Ok(())
}

/// Composite validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<(String, ValidationErrorType)>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, field: &str, error_type: ValidationErrorType) {
        self.is_valid = false;
        self.errors.push((field.to_string(), error_type));
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
            self.errors.extend(other.errors);
        }
    }
}

/// Validator for member registration
pub struct MemberValidator {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_numbers: Option<Vec<String>>,
    pub areas_of_interest: Option<Vec<String>>,
    pub postal_code: Option<String>,
}

impl MemberValidator {
    pub fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        if let Some(email) = &self.email {
            if let Err(err) = validate_email(email) {
                result.add_error("email", err);
            }
        } else {
            result.add_error("email", ValidationErrorType::RequiredField);
        }
        
        if let Some(password) = &self.password {
            if let Err(err) = validate_password(password) {
                result.add_error("password", err);
            }
        } else {
            result.add_error("password", ValidationErrorType::RequiredField);
        }
        
        if let Some(phone_numbers) = &self.phone_numbers {
            for (i, phone) in phone_numbers.iter().enumerate() {
                if let Err(err) = validate_phone(phone) {
                    result.add_error(&format!("phone_numbers[{}]", i), err);
                }
            }
            if let Err(err) = validate_no_duplicates(phone_numbers) {
                result.add_error("phone_numbers", err);
            }
        }
        
        if let Some(areas_of_interest) = &self.areas_of_interest {
            if let Err(err) = validate_no_duplicates(areas_of_interest) {
                result.add_error("areas_of_interest", err);
            }
        }
        
        if let Some(postal_code) = &self.postal_code {
            if let Err(err) = validate_postal_code(postal_code) {
                result.add_error("postal_code", err);
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid-email").is_err());
    }
    
    #[test]
    fn test_validate_password() {
        assert!(validate_password("Valid123!").is_ok());
        assert!(validate_password("weak").is_err());
    }
    
    #[test]
    fn test_validate_orcid() {
        assert!(validate_orcid("0000-0002-1825-0097").is_ok());
        assert!(validate_orcid("invalid").is_err());
    }
    
    #[test]
    fn test_validate_donation_percentages() {
        assert!(validate_donation_percentages(60, 20, 20).is_ok());
        assert!(validate_donation_percentages(50, 25, 25).is_err()); // charity < 60%
        assert!(validate_donation_percentages(60, 30, 20).is_err()); // total != 100%
    }
    
    #[test]
    fn test_validate_rating() {
        assert!(validate_rating(3).is_ok());
        assert!(validate_rating(0).is_err());
        assert!(validate_rating(6).is_err());
    }
    
    #[test]
    fn test_validate_keywords() {
        let keywords: Vec<String> = (0..10).map(|i| format!("keyword{}", i)).collect();
        assert!(validate_keywords(&keywords).is_ok());
        
        let too_many: Vec<String> = (0..11).map(|i| format!("keyword{}", i)).collect();
        assert!(validate_keywords(&too_many).is_err());
    }
}
