// backend/src/domain/value_objects/email.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq, Eq, Hash)]
pub struct Email {
    #[validate(email)]
    pub address: String,
    #[validate(length(max = 100))]
    pub label: Option<String>,
    pub is_primary: bool,
    pub is_verified: bool,
}

impl Email {
    pub fn new(address: String, is_primary: bool) -> Result<Self, String> {
        let email = Self {
            address,
            label: None,
            is_primary,
            is_verified: false,
        };
        
        email.validate()
            .map_err(|e| format!("Invalid email: {}", e))?;
        
        Ok(email)
    }
    
    pub fn normalize(&self) -> String {
        self.address.to_lowercase().trim().to_string()
    }
}
