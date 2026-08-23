use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::{CommitteeMembershipRole, CommitteeMembershipStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeMembership {
    pub membership_id: Uuid,
    pub member_id: Uuid,
    pub committee_id: Uuid,
    pub join_date: DateTime<Utc>,
    pub role: CommitteeMembershipRole,
    pub status: CommitteeMembershipStatus,
    pub term_end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CommitteeMembership {
    pub fn new(
        member_id: Uuid,
        committee_id: Uuid,
        role: CommitteeMembershipRole,
        term_end_date: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            membership_id: Uuid::new_v4(),
            member_id,
            committee_id,
            join_date: now,
            role,
            status: CommitteeMembershipStatus::Active,
            term_end_date,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_chair(&self) -> bool {
        matches!(self.role, CommitteeMembershipRole::Chair)
    }

    pub fn is_secretary(&self) -> bool {
        matches!(self.role, CommitteeMembershipRole::Secretary)
    }

    pub fn is_member(&self) -> bool {
        matches!(self.role, CommitteeMembershipRole::Member)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, CommitteeMembershipStatus::Active)
    }

    pub fn activate(&mut self) {
        self.status = CommitteeMembershipStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = CommitteeMembershipStatus::Inactive;
        self.updated_at = Utc::now();
    }

    pub fn promote_to_chair(&mut self) {
        self.role = CommitteeMembershipRole::Chair;
        self.updated_at = Utc::now();
    }

    pub fn demote_to_member(&mut self) {
        self.role = CommitteeMembershipRole::Member;
        self.updated_at = Utc::now();
    }

    pub fn is_term_expired(&self) -> bool {
        match self.term_end_date {
            Some(end_date) => Utc::now() > end_date,
            None => false,
        }
    }

    pub fn extend_term(&mut self, new_end_date: DateTime<Utc>) {
        self.term_end_date = Some(new_end_date);
        self.updated_at = Utc::now();
    }
}
