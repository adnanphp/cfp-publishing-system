// backend/src/domain/enums/member_status.rs
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum MemberStatus {
    Basic,
    Active,
    Donor,
    Premium,
    Suspended,
    Blacklisted,
}

impl Default for MemberStatus {
    fn default() -> Self {
        Self::Basic
    }
}

impl std::fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Active => write!(f, "active"),
            Self::Donor => write!(f, "donor"),
            Self::Premium => write!(f, "premium"),
            Self::Suspended => write!(f, "suspended"),
            Self::Blacklisted => write!(f, "blacklisted"),
        }
    }
}
