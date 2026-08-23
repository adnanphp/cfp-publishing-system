use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::TextStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVersion {
    pub version_id: Uuid,
    pub text_id: Uuid,
    pub changes: String,
    pub submitted_date: DateTime<Utc>,
    pub review_date: Option<DateTime<Utc>>,
    pub status: VersionStatus,
    pub change_summary: String,
    pub moderator_id: Option<Uuid>,
    pub version_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionStatus {
    Pending,
    Approved,
    Rejected,
}

impl Default for VersionStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl TextVersion {
    pub fn new(
        text_id: Uuid,
        changes: String,
        change_summary: String,
        version_number: i32,
    ) -> Self {
        Self {
            version_id: Uuid::new_v4(),
            text_id,
            changes,
            submitted_date: Utc::now(),
            review_date: None,
            status: VersionStatus::Pending,
            change_summary,
            moderator_id: None,
            version_number,
        }
    }

    pub fn approve(&mut self, moderator_id: Uuid) {
        self.status = VersionStatus::Approved;
        self.review_date = Some(Utc::now());
        self.moderator_id = Some(moderator_id);
    }

    pub fn reject(&mut self, moderator_id: Uuid) {
        self.status = VersionStatus::Rejected;
        self.review_date = Some(Utc::now());
        self.moderator_id = Some(moderator_id);
    }

    pub fn is_approved(&self) -> bool {
        self.status == VersionStatus::Approved
    }

    pub fn is_pending(&self) -> bool {
        self.status == VersionStatus::Pending
    }
}
