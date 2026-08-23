use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharityStatus {
    Active,
    Inactive,
    Pending,
    Suspended,
    Archived,
}

impl Default for CharityStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl CharityStatus {
    pub fn can_receive_donations(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_visible_to_public(&self) -> bool {
        matches!(self, Self::Active | Self::Archived)
    }

    pub fn is_under_review(&self) -> bool {
        matches!(self, Self::Pending)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, Self::Suspended)
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
            Self::Pending => "Pending",
            Self::Suspended => "Suspended",
            Self::Archived => "Archived",
        }
    }
}
