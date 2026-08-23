use crate::{
    domain::value_objects::{Email, PhoneNumber, Address},
    utils::error::AppError,
};

pub struct MemberValidator;

impl MemberValidator {
    pub fn validate_name(name: &str) -> Result<(), AppError> {
        if name.len() < 2 || name.len() > 100 {
            return Err(AppError::validation_error(
                "Name must be between 2 and 100 characters"
            ));
        }
        
        // Name should contain only letters, spaces, and common punctuation
        if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'' || c == '.') {
            return Err(AppError::validation_error(
                "Name contains invalid characters"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_organization(organization: &str) -> Result<(), AppError> {
        if organization.len() < 2 || organization.len() > 100 {
            return Err(AppError::validation_error(
                "Organization must be between 2 and 100 characters"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_email(email: &Email) -> Result<(), AppError> {
        // Email validation is already done in Email value object creation
        Ok(())
    }
    
    pub fn validate_password(password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::validation_error(
                "Password must be at least 8 characters"
            ));
        }
        
        // Check for common password patterns
        let weak_passwords = [
            "password", "12345678", "qwerty123", "admin123", "letmein",
            "welcome", "monkey", "dragon", "baseball", "football"
        ];
        
        if weak_passwords.contains(&password.to_lowercase().as_str()) {
            return Err(AppError::validation_error(
                "Password is too common. Please choose a stronger password."
            ));
        }
        
        // Check for at least one uppercase, one lowercase, one digit
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        
        if !(has_upper && has_lower && has_digit) {
            return Err(AppError::validation_error(
                "Password must contain at least one uppercase letter, one lowercase letter, and one digit"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_pseudonym(pseudonym: &Option<String>) -> Result<(), AppError> {
        if let Some(pseudo) = pseudonym {
            if pseudo.len() < 2 || pseudo.len() > 100 {
                return Err(AppError::validation_error(
                    "Pseudonym must be between 2 and 100 characters"
                ));
            }
            
            // Pseudonym cannot be the same as real name (would need actual name to compare)
            // This is just a basic check
            if pseudo.chars().all(|c| c.is_numeric()) {
                return Err(AppError::validation_error(
                    "Pseudonym cannot consist only of numbers"
                ));
            }
        }
        
        Ok(())
    }
    
    pub fn validate_phone_numbers(phone_numbers: &[PhoneNumber]) -> Result<(), AppError> {
        // Maximum 5 phone numbers per member
        if phone_numbers.len() > 5 {
            return Err(AppError::validation_error(
                "Maximum 5 phone numbers allowed per member"
            ));
        }
        
        // Validate each phone number
        for phone in phone_numbers {
            if !phone.validate_format() {
                return Err(AppError::validation_error(
                    format!("Invalid phone number format: {}", phone.full_number())
                ));
            }
        }
        
        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for phone in phone_numbers {
            let full_number = phone.full_number();
            if seen.contains(&full_number) {
                return Err(AppError::validation_error(
                    format!("Duplicate phone number: {}", full_number)
                ));
            }
            seen.insert(full_number);
        }
        
        // Exactly one primary phone number
        let primary_count = phone_numbers.iter().filter(|p| p.is_primary).count();
        if primary_count > 1 {
            return Err(AppError::validation_error(
                "Only one phone number can be marked as primary"
            ));
        }
        
        Ok(())
    }
    
    pub fn validate_areas_of_interest(areas: &[String]) -> Result<(), AppError> {
        // Maximum 10 areas of interest
        if areas.len() > 10 {
            return Err(AppError::validation_error(
                "Maximum 10 areas of interest allowed"
            ));
        }
        
        // Each area must be between 2 and 50 characters
        for area in areas {
            if area.len() < 2 || area.len() > 50 {
                return Err(AppError::validation_error(
                    "Each area of interest must be between 2 and 50 characters"
                ));
            }
        }
        
        // No duplicates
        let mut seen = std::collections::HashSet::new();
        for area in areas {
            let lower_area = area.to_lowercase();
            if seen.contains(&lower_area) {
                return Err(AppError::validation_error(
                    format!("Duplicate area of interest: {}", area)
                ));
            }
            seen.insert(lower_area);
        }
        
        Ok(())
    }
    
    pub fn validate_address(address: &Address) -> Result<(), AppError> {
        // Street validation
        if address.street.len() < 5 || address.street.len() > 100 {
            return Err(AppError::validation_error(
                "Street must be between 5 and 100 characters"
            ));
        }
        
        // City validation
        if address.city.len() < 2 || address.city.len() > 50 {
            return Err(AppError::validation_error(
                "City must be between 2 and 50 characters"
            ));
        }
        
        // State validation
        if address.state.len() < 2 || address.state.len() > 50 {
            return Err(AppError::validation_error(
                "State must be between 2 and 50 characters"
            ));
        }
        
        // Country validation
        if address.country.len() < 2 || address.country.len() > 50 {
            return Err(AppError::validation_error(
                "Country must be between 2 and 50 characters"
            ));
        }
        
        // Postal code validation
        if address.postal_code.len() < 3 || address.postal_code.len() > 20 {
            return Err(AppError::validation_error(
                "Postal code must be between 3 and 20 characters"
            ));
        }
        
        Ok(())
    }
}
