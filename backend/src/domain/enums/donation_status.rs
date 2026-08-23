// backend/src/domain/enums/donation_status.rs
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum DonationStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

impl Default for DonationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for DonationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Processing => write!(f, "processing"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Refunded => write!(f, "refunded"),
        }
    }
}
