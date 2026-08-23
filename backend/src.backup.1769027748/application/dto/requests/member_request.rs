use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::value_objects::{Address, PhoneNumber};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateMemberRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    
    #[validate(length(min = 2, max = 100))]
    pub organization: String,
    
    #[validate(email)]
    pub primary_email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
    
    pub pseudonym: Option<String>,
    pub recovery_email: Option<String>,
    pub address: Option<Address>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub areas_of_interest: Vec<String>,
    pub introduced_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    
    #[validate(length(min = 2, max = 100))]
    pub organization: Option<String>,
    
    pub pseudonym: Option<String>,
    pub recovery_email: Option<String>,
    pub address: Option<Address>,
    pub phone_numbers: Option<Vec<PhoneNumber>>,
    pub areas_of_interest: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAuthorRequest {
    #[validate(length(min = 16, max = 19))]
    pub orcid: String,
    
    #[validate(length(min = 10, max = 2000))]
    pub bio: String,
    
    #[validate(length(min = 2, max = 100))]
    pub specialization: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAdminRequest {
    pub role: String, // "super", "content", "financial"
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateModeratorRequest {
    #[validate(length(min = 2, max = 50))]
    pub domain: String,
    
    pub expertise_areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateMemberStatusRequest {
    pub status: String, // "active", "suspended", "banned", "inactive"
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchMembersRequest {
    pub query: Option<String>,
    pub status: Option<String>,
    pub role: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BulkImportMembersRequest {
    pub members: Vec<CreateMemberRequest>,
    pub send_welcome_emails: bool,
}
