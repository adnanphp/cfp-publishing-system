use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::{CommitteeScope, CommitteeStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Committee {
    pub committee_id: Uuid,
    pub name: String,
    pub purpose: String,
    pub scope: CommitteeScope,
    pub formation_date: DateTime<Utc>,
    pub status: CommitteeStatus,
    pub member_count: i32,
    pub chair_member_id: Option<Uuid>,
    pub secretary_member_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Committee {
    pub fn new(
        name: String,
        purpose: String,
        scope: CommitteeScope,
        chair_member_id: Option<Uuid>,
        secretary_member_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            committee_id: Uuid::new_v4(),
            name,
            purpose,
            scope,
            formation_date: now,
            status: CommitteeStatus::Active,
            member_count: 0,
            chair_member_id,
            secretary_member_id,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn activate(&mut self) {
        self.status = CommitteeStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = CommitteeStatus::Inactive;
        self.updated_at = Utc::now();
    }

    pub fn increment_member_count(&mut self) {
        self.member_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_member_count(&mut self) {
        self.member_count = (self.member_count - 1).max(0);
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        self.status == CommitteeStatus::Active
    }

    pub fn can_handle_plagiarism(&self) -> bool {
        matches!(self.scope, CommitteeScope::Plagiarism)
    }

    pub fn can_handle_content(&self) -> bool {
        matches!(self.scope, CommitteeScope::Content)
    }

    pub fn can_handle_finance(&self) -> bool {
        matches!(self.scope, CommitteeScope::Finance)
    }

    pub fn can_handle_appeals(&self) -> bool {
        matches!(self.scope, CommitteeScope::Appeals)
    }
}
