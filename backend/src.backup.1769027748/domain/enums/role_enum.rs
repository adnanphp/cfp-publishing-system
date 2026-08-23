use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleEnum {
    Super,
    Content,
    Financial,
}

impl RoleEnum {
    pub fn permissions(&self) -> Vec<&'static str> {
        match self {
            Self::Super => vec![
                "manage_all_users",
                "manage_all_content",
                "manage_all_finances",
                "manage_system_config",
                "assign_roles",
                "view_audit_logs",
                "override_decisions",
            ],
            Self::Content => vec![
                "create_content",
                "edit_content",
                "delete_content",
                "moderate_content",
                "approve_texts",
                "manage_categories",
                "view_content_analytics",
            ],
            Self::Financial => vec![
                "view_donations",
                "process_donations",
                "generate_financial_reports",
                "manage_charities",
                "view_revenue",
                "export_financial_data",
                "manage_payouts",
            ],
        }
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, Self::Super)
    }

    pub fn can_manage_content(&self) -> bool {
        matches!(self, Self::Super | Self::Content)
    }

    pub fn can_manage_finances(&self) -> bool {
        matches!(self, Self::Super | Self::Financial)
    }

    pub fn dashboard_access(&self) -> Vec<&'static str> {
        match self {
            Self::Super => vec!["admin", "content", "finance", "system", "analytics"],
            Self::Content => vec!["content", "analytics"],
            Self::Financial => vec!["finance", "analytics"],
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "super" | "superadmin" | "administrator" => Some(Self::Super),
            "content" | "contentadmin" | "editor" => Some(Self::Content),
            "financial" | "financialadmin" | "finance" => Some(Self::Financial),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Super => "Super",
            Self::Content => "Content",
            Self::Financial => "Financial",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Super => "Super Administrator",
            Self::Content => "Content Administrator",
            Self::Financial => "Financial Administrator",
        }
    }
}
