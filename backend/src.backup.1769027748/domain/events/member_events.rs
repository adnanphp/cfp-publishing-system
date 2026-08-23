use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::DomainEvent;
use crate::domain::enums::RoleEnum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberEvent {
    MemberCreated {
        member_id: Uuid,
        name: String,
        email: String,
        timestamp: DateTime<Utc>,
    },
    MemberActivated {
        member_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    MemberSuspended {
        member_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    MemberBanned {
        member_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    AuthorRegistered {
        member_id: Uuid,
        orcid: String,
        timestamp: DateTime<Utc>,
    },
    AdminRegistered {
        member_id: Uuid,
        role: RoleEnum,
        timestamp: DateTime<Utc>,
    },
    ModeratorRegistered {
        member_id: Uuid,
        domain: String,
        timestamp: DateTime<Utc>,
    },
    VerificationMatrixUpdated {
        member_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent for MemberEvent {
    fn event_type(&self) -> &'static str {
        match self {
            MemberEvent::MemberCreated { .. } => "MemberCreated",
            MemberEvent::MemberActivated { .. } => "MemberActivated",
            MemberEvent::MemberSuspended { .. } => "MemberSuspended",
            MemberEvent::MemberBanned { .. } => "MemberBanned",
            MemberEvent::AuthorRegistered { .. } => "AuthorRegistered",
            MemberEvent::AdminRegistered { .. } => "AdminRegistered",
            MemberEvent::ModeratorRegistered { .. } => "ModeratorRegistered",
            MemberEvent::VerificationMatrixUpdated { .. } => "VerificationMatrixUpdated",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            MemberEvent::MemberCreated { member_id, .. } => *member_id,
            MemberEvent::MemberActivated { member_id, .. } => *member_id,
            MemberEvent::MemberSuspended { member_id, .. } => *member_id,
            MemberEvent::MemberBanned { member_id, .. } => *member_id,
            MemberEvent::AuthorRegistered { member_id, .. } => *member_id,
            MemberEvent::AdminRegistered { member_id, .. } => *member_id,
            MemberEvent::ModeratorRegistered { member_id, .. } => *member_id,
            MemberEvent::VerificationMatrixUpdated { member_id, .. } => *member_id,
        }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        match self {
            MemberEvent::MemberCreated { timestamp, .. } => *timestamp,
            MemberEvent::MemberActivated { timestamp, .. } => *timestamp,
            MemberEvent::MemberSuspended { timestamp, .. } => *timestamp,
            MemberEvent::MemberBanned { timestamp, .. } => *timestamp,
            MemberEvent::AuthorRegistered { timestamp, .. } => *timestamp,
            MemberEvent::AdminRegistered { timestamp, .. } => *timestamp,
            MemberEvent::ModeratorRegistered { timestamp, .. } => *timestamp,
            MemberEvent::VerificationMatrixUpdated { timestamp, .. } => *timestamp,
        }
    }
}
