// backend/src/domain/enums/text_status.rs
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum TextStatus {
    Draft,
    UnderReview,
    Published,
    Archived,
}

impl Default for TextStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl std::fmt::Display for TextStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Published => write!(f, "published"),
            Self::Archived => write!(f, "archived"),
        }
    }
}
