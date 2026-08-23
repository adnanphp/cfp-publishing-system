use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::DomainEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextEvent {
    TextCreated {
        text_id: Uuid,
        author_orcid: String,
        title: String,
        timestamp: DateTime<Utc>,
    },
    SubmittedForReview {
        text_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Published {
        text_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Archived {
        text_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    VersionCreated {
        text_id: Uuid,
        version_number: i32,
        timestamp: DateTime<Utc>,
    },
    VersionApproved {
        text_id: Uuid,
        version_id: Uuid,
        moderator_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    VersionRejected {
        text_id: Uuid,
        version_id: Uuid,
        moderator_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Downloaded {
        text_id: Uuid,
        member_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    Donated {
        text_id: Uuid,
        member_id: Uuid,
        amount: f64,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent for TextEvent {
    fn event_type(&self) -> &'static str {
        match self {
            TextEvent::TextCreated { .. } => "TextCreated",
            TextEvent::SubmittedForReview { .. } => "SubmittedForReview",
            TextEvent::Published { .. } => "Published",
            TextEvent::Archived { .. } => "Archived",
            TextEvent::VersionCreated { .. } => "VersionCreated",
            TextEvent::VersionApproved { .. } => "VersionApproved",
            TextEvent::VersionRejected { .. } => "VersionRejected",
            TextEvent::Downloaded { .. } => "Downloaded",
            TextEvent::Donated { .. } => "Donated",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            TextEvent::TextCreated { text_id, .. } => *text_id,
            TextEvent::SubmittedForReview { text_id, .. } => *text_id,
            TextEvent::Published { text_id, .. } => *text_id,
            TextEvent::Archived { text_id, .. } => *text_id,
            TextEvent::VersionCreated { text_id, .. } => *text_id,
            TextEvent::VersionApproved { text_id, .. } => *text_id,
            TextEvent::VersionRejected { text_id, .. } => *text_id,
            TextEvent::Downloaded { text_id, .. } => *text_id,
            TextEvent::Donated { text_id, .. } => *text_id,
        }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        match self {
            TextEvent::TextCreated { timestamp, .. } => *timestamp,
            TextEvent::SubmittedForReview { timestamp, .. } => *timestamp,
            TextEvent::Published { timestamp, .. } => *timestamp,
            TextEvent::Archived { timestamp, .. } => *timestamp,
            TextEvent::VersionCreated { timestamp, .. } => *timestamp,
            TextEvent::VersionApproved { timestamp, .. } => *timestamp,
            TextEvent::VersionRejected { timestamp, .. } => *timestamp,
            TextEvent::Downloaded { timestamp, .. } => *timestamp,
            TextEvent::Donated { timestamp, .. } => *timestamp,
        }
    }
}
