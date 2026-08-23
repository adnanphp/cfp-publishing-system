// backend/src/domain/value_objects/address.rs
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct Address {
    #[validate(length(max = 100))]
    pub street: Option<String>,
    
    #[validate(length(max = 50))]
    pub city: Option<String>,
    
    #[validate(length(max = 50))]
    pub state: Option<String>,
    
    #[validate(length(max = 50))]
    pub country: Option<String>,
    
    #[validate(length(max = 20))]
    pub postal_code: Option<String>,
}

impl Address {
    pub fn to_string(&self) -> String {
        let parts: Vec<&str> = vec![
            self.street.as_deref().unwrap_or(""),
            self.city.as_deref().unwrap_or(""),
            self.state.as_deref().unwrap_or(""),
            self.country.as_deref().unwrap_or(""),
            self.postal_code.as_deref().unwrap_or(""),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
        
        parts.join(", ")
    }
    
    pub fn is_complete(&self) -> bool {
        self.street.is_some()
            && self.city.is_some()
            && self.country.is_some()
            && self.postal_code.is_some()
    }
}
