use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct PhoneNumber {
    #[validate(length(min = 10, max = 15))]
    pub number: String,
    
    pub country_code: String,
    
    #[serde(rename = "type")]
    pub phone_type: PhoneType,
    
    pub is_primary: bool,
    pub is_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PhoneType {
    Mobile,
    Home,
    Work,
    Fax,
    Other,
}

impl PhoneNumber {
    pub fn new(number: String, country_code: String, phone_type: PhoneType, is_primary: bool) -> Self {
        Self {
            number,
            country_code,
            phone_type,
            is_primary,
            is_verified: false,
        }
    }

    pub fn full_number(&self) -> String {
        format!("+{}{}", self.country_code, self.number)
    }

    pub fn format_for_display(&self) -> String {
        match self.phone_type {
            PhoneType::Mobile => format!("Mobile: {}", self.full_number()),
            PhoneType::Home => format!("Home: {}", self.full_number()),
            PhoneType::Work => format!("Work: {}", self.full_number()),
            PhoneType::Fax => format!("Fax: {}", self.full_number()),
            PhoneType::Other => format!("Other: {}", self.full_number()),
        }
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
    }

    pub fn mark_as_primary(&mut self) {
        self.is_primary = true;
    }

    pub fn mark_as_secondary(&mut self) {
        self.is_primary = false;
    }

    pub fn validate_format(&self) -> bool {
        // Remove all non-digit characters
        let digits: String = self.number.chars().filter(|c| c.is_digit(10)).collect();
        
        // Basic validation: check length and that it contains only digits
        digits.len() >= 7 && digits.len() <= 15 && digits.chars().all(|c| c.is_digit(10))
    }

    pub fn sanitize(&mut self) {
        // Remove all non-digit characters except leading plus
        self.number = self.number.chars()
            .filter(|c| c.is_digit(10))
            .collect();
    }
}

impl Default for PhoneType {
    fn default() -> Self {
        Self::Mobile
    }
}
