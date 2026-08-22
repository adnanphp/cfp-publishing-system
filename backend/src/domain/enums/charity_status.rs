use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharityStatus {
    Active,
    Inactive,
    Pending,
}

impl CharityStatus {
    pub fn from_db(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "active" => CharityStatus::Active,
            "inactive" => CharityStatus::Inactive,
            "pending" => CharityStatus::Pending,
            _ => CharityStatus::Pending,
        }
    }
    
    pub fn to_db(&self) -> &'static str {
        match self {
            CharityStatus::Active => "active",
            CharityStatus::Inactive => "inactive",
            CharityStatus::Pending => "pending",
        }
    }
    
    pub fn can_receive_donations(&self) -> bool {
        matches!(self, CharityStatus::Active)
    }
}
