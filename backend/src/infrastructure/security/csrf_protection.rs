// Simple stub for CSRF protection
pub struct CsrfProtection;

impl CsrfProtection {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_token(&self) -> String {
        "csrf_token".to_string()
    }
    
    pub fn validate_token(&self, _token: &str) -> bool {
        true
    }
}
