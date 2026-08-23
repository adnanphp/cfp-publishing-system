use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::RoleEnum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub admin_id: Uuid,
    pub member_id: Uuid,
    pub role: RoleEnum,
    pub permissions: Vec<String>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Admin {
    pub fn new(member_id: Uuid, role: RoleEnum) -> Self {
        let permissions = match role {
            RoleEnum::Super => vec![
                "manage_users".to_string(),
                "manage_content".to_string(),
                "manage_finances".to_string(),
                "manage_system".to_string(),
            ],
            RoleEnum::Content => vec![
                "manage_content".to_string(),
                "moderate_texts".to_string(),
                "approve_versions".to_string(),
            ],
            RoleEnum::Financial => vec![
                "view_finances".to_string(),
                "process_donations".to_string(),
                "generate_reports".to_string(),
            ],
        };

        let now = chrono::Utc::now();
        Self {
            admin_id: Uuid::new_v4(),
            member_id,
            role,
            permissions,
            last_login: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
            self.updated_at = chrono::Utc::now();
        }
    }

    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
        self.updated_at = chrono::Utc::now();
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    pub fn is_super_admin(&self) -> bool {
        matches!(self.role, RoleEnum::Super)
    }

    pub fn is_content_admin(&self) -> bool {
        matches!(self.role, RoleEnum::Content)
    }

    pub fn is_financial_admin(&self) -> bool {
        matches!(self.role, RoleEnum::Financial)
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = chrono::Utc::now();
    }
}
