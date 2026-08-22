use crate::domain::models::Member;

#[derive(Debug, Clone)]
pub struct Admin {
    pub member: Member,
    pub permissions: Vec<String>,
}

impl Admin {
    pub fn new(member: Member) -> Self {
        Self {
            member,
            permissions: vec!["manage_users".to_string(), "manage_content".to_string()],
        }
    }
    
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}
