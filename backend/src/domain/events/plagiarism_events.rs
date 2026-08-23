use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::DomainEvent;
use crate::domain::enums::{PlagiarismResolution, VoteType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlagiarismEvent {
    CaseOpened {
        case_id: Uuid,
        text_id: Uuid,
        committee_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ReviewStarted {
        case_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    VotingStarted {
        case_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    CaseResolved {
        case_id: Uuid,
        resolution: PlagiarismResolution,
        timestamp: DateTime<Utc>,
    },
    CaseAppealed {
        case_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    VoteCast {
        case_id: Uuid,
        member_id: Uuid,
        vote_type: VoteType,
        timestamp: DateTime<Utc>,
    },
    PlagiarismConfirmed {
        case_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent for PlagiarismEvent {
    fn event_type(&self) -> &'static str {
        match self {
            PlagiarismEvent::CaseOpened { .. } => "CaseOpened",
            PlagiarismEvent::ReviewStarted { .. } => "ReviewStarted",
            PlagiarismEvent::VotingStarted { .. } => "VotingStarted",
            PlagiarismEvent::CaseResolved { .. } => "CaseResolved",
            PlagiarismEvent::CaseAppealed { .. } => "CaseAppealed",
            PlagiarismEvent::VoteCast { .. } => "VoteCast",
            PlagiarismEvent::PlagiarismConfirmed { .. } => "PlagiarismConfirmed",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            PlagiarismEvent::CaseOpened { case_id, .. } => *case_id,
            PlagiarismEvent::ReviewStarted { case_id, .. } => *case_id,
            PlagiarismEvent::VotingStarted { case_id, .. } => *case_id,
            PlagiarismEvent::CaseResolved { case_id, .. } => *case_id,
            PlagiarismEvent::CaseAppealed { case_id, .. } => *case_id,
            PlagiarismEvent::VoteCast { case_id, .. } => *case_id,
            PlagiarismEvent::PlagiarismConfirmed { case_id, .. } => *case_id,
        }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        match self {
            PlagiarismEvent::CaseOpened { timestamp, .. } => *timestamp,
            PlagiarismEvent::ReviewStarted { timestamp, .. } => *timestamp,
            PlagiarismEvent::VotingStarted { timestamp, .. } => *timestamp,
            PlagiarismEvent::CaseResolved { timestamp, .. } => *timestamp,
            PlagiarismEvent::CaseAppealed { timestamp, .. } => *timestamp,
            PlagiarismEvent::VoteCast { timestamp, .. } => *timestamp,
            PlagiarismEvent::PlagiarismConfirmed { timestamp, .. } => *timestamp,
        }
    }
}
