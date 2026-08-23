use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::DomainEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DonationEvent {
    DonationCreated {
        donation_id: Uuid,
        member_id: Uuid,
        text_id: Uuid,
        charity_id: Uuid,
        amount: f64,
        timestamp: DateTime<Utc>,
    },
    PaymentProcessed {
        donation_id: Uuid,
        transaction_id: String,
        timestamp: DateTime<Utc>,
    },
    PaymentFailed {
        donation_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    DistributionProcessed {
        donation_id: Uuid,
        charity_amount: f64,
        cfp_amount: f64,
        author_amount: f64,
        timestamp: DateTime<Utc>,
    },
    DistributionFailed {
        donation_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    Refunded {
        donation_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent for DonationEvent {
    fn event_type(&self) -> &'static str {
        match self {
            DonationEvent::DonationCreated { .. } => "DonationCreated",
            DonationEvent::PaymentProcessed { .. } => "PaymentProcessed",
            DonationEvent::PaymentFailed { .. } => "PaymentFailed",
            DonationEvent::DistributionProcessed { .. } => "DistributionProcessed",
            DonationEvent::DistributionFailed { .. } => "DistributionFailed",
            DonationEvent::Refunded { .. } => "Refunded",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            DonationEvent::DonationCreated { donation_id, .. } => *donation_id,
            DonationEvent::PaymentProcessed { donation_id, .. } => *donation_id,
            DonationEvent::PaymentFailed { donation_id, .. } => *donation_id,
            DonationEvent::DistributionProcessed { donation_id, .. } => *donation_id,
            DonationEvent::DistributionFailed { donation_id, .. } => *donation_id,
            DonationEvent::Refunded { donation_id, .. } => *donation_id,
        }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        match self {
            DonationEvent::DonationCreated { timestamp, .. } => *timestamp,
            DonationEvent::PaymentProcessed { timestamp, .. } => *timestamp,
            DonationEvent::PaymentFailed { timestamp, .. } => *timestamp,
            DonationEvent::DistributionProcessed { timestamp, .. } => *timestamp,
            DonationEvent::DistributionFailed { timestamp, .. } => *timestamp,
            DonationEvent::Refunded { timestamp, .. } => *timestamp,
        }
    }
}
