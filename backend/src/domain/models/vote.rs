use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::VoteType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub vote_id: Uuid,
    pub member_id: Uuid,
    pub case_id: Uuid,
    pub vote: VoteType,
    pub date: DateTime<Utc>,
    pub rationale: Option<String>,
    pub confidence_level: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vote {
    pub fn new(
        member_id: Uuid,
        case_id: Uuid,
        vote: VoteType,
        rationale: Option<String>,
        confidence_level: Option<i32>,
    ) -> Self {
        let now = Utc::now();
        Self {
            vote_id: Uuid::new_v4(),
            member_id,
            case_id,
            vote,
            date: now,
            rationale,
            confidence_level,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_plagiarized_vote(&self) -> bool {
        matches!(self.vote, VoteType::Plagiarized)
    }

    pub fn is_not_plagiarized_vote(&self) -> bool {
        matches!(self.vote, VoteType::NotPlagiarized)
    }

    pub fn is_abstain_vote(&self) -> bool {
        matches!(self.vote, VoteType::Abstain)
    }

    pub fn update_vote(&mut self, new_vote: VoteType, rationale: Option<String>) {
        self.vote = new_vote;
        self.rationale = rationale;
        self.updated_at = Utc::now();
    }

    pub fn validate_confidence_level(&self) -> bool {
        match self.confidence_level {
            Some(level) => level >= 1 && level <= 10,
            None => true,
        }
    }
}
