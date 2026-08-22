use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStatus {
    Draft,
    UnderReview,
    Published,
    Archived,
}

impl TextStatus {
    pub fn from_db(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "draft" => TextStatus::Draft,
            "under_review" => TextStatus::UnderReview,
            "published" => TextStatus::Published,
            "archived" => TextStatus::Archived,
            _ => TextStatus::Draft,
        }
    }
    
    pub fn to_db(&self) -> &'static str {
        match self {
            TextStatus::Draft => "draft",
            TextStatus::UnderReview => "under_review",
            TextStatus::Published => "published",
            TextStatus::Archived => "archived",
        }
    }
}
