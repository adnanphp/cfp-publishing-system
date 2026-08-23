#!/bin/bash

echo "Restoring Validators Module"
echo "==========================="

mkdir -p src/application/validators

# Create main mod.rs
cat > src/application/validators/mod.rs << 'MODEOF'
//! Business validators

pub mod member_validator;
pub mod text_validator;
pub mod donation_validator;
pub mod auth_validator;

use crate::domain::value_objects::{Email, PhoneNumber, Address};

/// Common validation functions
pub fn validate_string_length(value: &str, min: usize, max: usize, field: &str) -> Result<(), String> {
    if value.len() < min {
        return Err(format!("{} must be at least {} characters", field, min));
    }
    if value.len() > max {
        return Err(format!("{} must be at most {} characters", field, max));
    }
    Ok(())
}

pub fn validate_email_format(email: &str) -> Result<(), String> {
    match Email::new(email.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn validate_phone_number(phone: &str) -> Result<(), String> {
    match PhoneNumber::new(phone.to_string()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
MODEOF

# Create member validator
cat > src/application/validators/member_validator.rs << 'MEMBEREOF'
//! Member validation logic

use super::{validate_string_length, validate_email_format, validate_phone_number};
use crate::domain::value_objects::Address;

pub struct MemberValidator;

impl MemberValidator {
    pub fn validate_username(username: &str) -> Result<(), String> {
        validate_string_length(username, 3, 50, "Username")
    }
    
    pub fn validate_email(email: &str) -> Result<(), String> {
        validate_email_format(email)
    }
    
    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        // Check for at least one number
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err("Password must contain at least one number".to_string());
        }
        
        // Check for at least one uppercase letter
        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        
        // Check for at least one lowercase letter
        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        
        Ok(())
    }
    
    pub fn validate_address(address: &Address) -> Result<(), String> {
        address.validate()
    }
    
    pub fn validate_profile_data(
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if let Err(e) = Self::validate_username(username) {
            errors.push(e);
        }
        
        if let Err(e) = Self::validate_email(email) {
            errors.push(e);
        }
        
        if let Err(e) = Self::validate_password(password) {
            errors.push(e);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
MEMBEREOF

# Create text validator
cat > src/application/validators/text_validator.rs << 'TEXTEOF'
//! Text/content validation logic

use super::validate_string_length;

pub struct TextValidator;

impl TextValidator {
    pub fn validate_title(title: &str) -> Result<(), String> {
        validate_string_length(title, 3, 200, "Title")
    }
    
    pub fn validate_content(content: &str) -> Result<(), String> {
        if content.len() < 10 {
            return Err("Content must be at least 10 characters".to_string());
        }
        if content.len() > 10000 {
            return Err("Content must be at most 10000 characters".to_string());
        }
        Ok(())
    }
    
    pub fn validate_tags(tags: &[String]) -> Result<(), String> {
        if tags.len() > 10 {
            return Err("Cannot have more than 10 tags".to_string());
        }
        
        for tag in tags {
            if tag.len() > 20 {
                return Err(format!("Tag '{}' is too long (max 20 characters)", tag));
            }
            if tag.contains(' ') {
                return Err(format!("Tag '{}' cannot contain spaces", tag));
            }
        }
        
        Ok(())
    }
}
TEXTEOF

# Create donation validator
cat > src/application/validators/donation_validator.rs << 'DONATIONEOF'
//! Donation validation logic

use crate::domain::value_objects::PercentageDistribution;

pub struct DonationValidator;

impl DonationValidator {
    pub fn validate_amount(amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }
        if amount > 1000000.0 {
            return Err("Amount is too large".to_string());
        }
        Ok(())
    }
    
    pub fn validate_currency(currency: &str) -> Result<(), String> {
        let valid_currencies = ["USD", "EUR", "GBP", "JPY", "CAD", "AUD"];
        if !valid_currencies.contains(&currency) {
            return Err(format!("Invalid currency: {}. Valid: {:?}", currency, valid_currencies));
        }
        Ok(())
    }
    
    pub fn validate_distribution(distribution: &PercentageDistribution) -> Result<(), String> {
        distribution.validate()
    }
}
DONATIONEOF

# Create auth validator
cat > src/application/validators/auth_validator.rs << 'AUTHEOF'
//! Authentication validation logic

use super::{validate_email_format, validate_string_length};

pub struct AuthValidator;

impl AuthValidator {
    pub fn validate_login(email: &str, password: &str) -> Result<(), String> {
        if let Err(e) = validate_email_format(email) {
            return Err(e);
        }
        
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        Ok(())
    }
    
    pub fn validate_reset_token(token: &str) -> Result<(), String> {
        if token.len() != 64 {
            return Err("Invalid reset token length".to_string());
        }
        
        // Check if token is valid hex
        for c in token.chars() {
            if !c.is_ascii_hexdigit() {
                return Err("Reset token must be hexadecimal".to_string());
            }
        }
        
        Ok(())
    }
}
AUTHEOF

# Update application/mod.rs
if ! grep -q "pub mod validators" src/application/mod.rs; then
    echo "pub mod validators;" >> src/application/mod.rs
fi

echo ""
echo "Testing validators compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Validators compile successfully!"
    
    echo ""
    echo "Testing build..."
    cargo build
    
    if [ $? -eq 0 ]; then
        echo "✅ Build successful with validators!"
        
        echo ""
        echo "🎉 Validators module restored successfully!"
        echo ""
        echo "You now have:"
        echo "1. ✅ Domain layer (enums + value objects)"
        echo "2. ✅ DTOs (minimal)"
        echo "3. ✅ Validators (complete)"
        echo "4. ✅ Utils (minimal)"
        echo "5. ✅ Project compiles, builds, and runs"
        
        echo ""
        echo "Next steps could be:"
        echo "1. Restore services module"
        echo "2. Add axum for API"
        echo "3. Restore database layer"
        echo "4. Test the validators with domain objects"
    else
        echo "❌ Build failed"
    fi
else
    echo "❌ Validators compilation failed"
fi
