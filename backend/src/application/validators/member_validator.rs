pub struct MemberValidator;

impl MemberValidator {
    pub fn validate_email(email: &str) -> Result<(), String> {
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Email must contain @ symbol".to_string());
        }
        
        Ok(())
    }
    
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if name.len() < 2 {
            return Err("Name must be at least 2 characters".to_string());
        }
        
        Ok(())
    }
}
