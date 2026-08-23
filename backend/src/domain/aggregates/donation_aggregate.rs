use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::{
    models::{Donation, Text, Member},
    enums::DonationStatus,
    value_objects::PercentageDistribution,
    events::donation_events::DonationEvent,
};

#[derive(Debug, Clone)]
pub struct DonationAggregate {
    pub donation: Donation,
    pub text: Option<Text>,
    pub charity: Option<String>,
    pub donor: Option<Member>,
    pub distribution_history: Vec<DistributionRecord>,
    pub pending_events: Vec<DonationEvent>,
}

#[derive(Debug, Clone)]
pub struct DistributionRecord {
    pub record_id: Uuid,
    pub donation_id: Uuid,
    pub charity_amount: f64,
    pub cfp_amount: f64,
    pub author_amount: f64,
    pub distribution_date: DateTime<Utc>,
    pub transaction_id: Option<String>,
    pub status: DistributionStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionStatus {
    Pending,
    Processed,
    Failed,
    Refunded,
}

impl DonationAggregate {
    pub fn new(
        member_id: Uuid,
        text_id: Uuid,
        charity_id: Uuid,
        amount: f64,
        currency: String,
        payment_method: String,
        distribution: PercentageDistribution,
    ) -> Self {
        let donation = Donation::new(
            member_id,
            text_id,
            charity_id,
            amount,
            currency,
            payment_method,
            distribution,
        );

        Self {
            donation,
            text: None,
            charity: None,
            donor: None,
            distribution_history: Vec::new(),
            pending_events: Vec::new(),
        }
    }

    pub fn donation_id(&self) -> Uuid {
        self.donation.donation_id
    }

    pub fn process_payment(&mut self, transaction_id: String) {
        if self.donation.status == DonationStatus::Pending {
            self.donation.status = DonationStatus::Completed;
            self.donation.transaction_id = Some(transaction_id.clone());
            
            self.pending_events.push(DonationEvent::PaymentProcessed {
                donation_id: self.donation_id(),
                transaction_id,
                timestamp: Utc::now(),
            });

            // Create distribution record
            self.create_distribution_record();
        }
    }

    pub fn create_distribution_record(&mut self) {
        let charity_amount = self.donation.amount * (self.donation.charity_pct as f64 / 100.0);
        let cfp_amount = self.donation.amount * (self.donation.cfp_pct as f64 / 100.0);
        let author_amount = self.donation.amount * (self.donation.author_pct as f64 / 100.0);

        let record = DistributionRecord {
            record_id: Uuid::new_v4(),
            donation_id: self.donation_id(),
            charity_amount,
            cfp_amount,
            author_amount,
            distribution_date: Utc::now(),
            transaction_id: self.donation.transaction_id.clone(),
            status: DistributionStatus::Pending,
        };

        self.distribution_history.push(record);
    }

    pub fn mark_distribution_processed(&mut self) {
        if let Some(record) = self.distribution_history.last_mut() {
            if record.status == DistributionStatus::Pending {
                record.status = DistributionStatus::Processed;
                
                self.pending_events.push(DonationEvent::DistributionProcessed {
                    donation_id: self.donation_id(),
                    charity_amount: record.charity_amount,
                    cfp_amount: record.cfp_amount,
                    author_amount: record.author_amount,
                    timestamp: Utc::now(),
                });
            }
        }
    }

    pub fn mark_distribution_failed(&mut self, reason: String) {
        if let Some(record) = self.distribution_history.last_mut() {
            if record.status == DistributionStatus::Pending {
                record.status = DistributionStatus::Failed;
                
                self.pending_events.push(DonationEvent::DistributionFailed {
                    donation_id: self.donation_id(),
                    reason,
                    timestamp: Utc::now(),
                });
            }
        }
    }

    pub fn refund(&mut self, reason: String) {
        if self.donation.status == DonationStatus::Completed {
            self.donation.status = DonationStatus::Refunded;
            
            // Refund all distributions if they were processed
            for record in &mut self.distribution_history {
                if record.status == DistributionStatus::Processed {
                    record.status = DistributionStatus::Refunded;
                }
            }
            
            self.pending_events.push(DonationEvent::Refunded {
                donation_id: self.donation_id(),
                reason,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn fail_payment(&mut self, reason: String) {
        if self.donation.status == DonationStatus::Pending {
            self.donation.status = DonationStatus::Failed;
            
            self.pending_events.push(DonationEvent::PaymentFailed {
                donation_id: self.donation_id(),
                reason,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn get_total_distributed_to_charity(&self) -> f64 {
        self.distribution_history.iter()
            .filter(|r| r.status == DistributionStatus::Processed)
            .map(|r| r.charity_amount)
            .sum()
    }

    pub fn get_total_distributed_to_cfp(&self) -> f64 {
        self.distribution_history.iter()
            .filter(|r| r.status == DistributionStatus::Processed)
            .map(|r| r.cfp_amount)
            .sum()
    }

    pub fn get_total_distributed_to_author(&self) -> f64 {
        self.distribution_history.iter()
            .filter(|r| r.status == DistributionStatus::Processed)
            .map(|r| r.author_amount)
            .sum()
    }

    pub fn is_completed(&self) -> bool {
        self.donation.status == DonationStatus::Completed
    }

    pub fn is_pending(&self) -> bool {
        self.donation.status == DonationStatus::Pending
    }

    pub fn is_failed(&self) -> bool {
        self.donation.status == DonationStatus::Failed
    }

    pub fn is_refunded(&self) -> bool {
        self.donation.status == DonationStatus::Refunded
    }

    pub fn validate_distribution(&self) -> bool {
        self.donation.validate_distribution()
    }

    pub fn get_distribution_summary(&self) -> String {
        format!(
            "Charity: {}% (${:.2}), CFP: {}% (${:.2}), Author: {}% (${:.2})",
            self.donation.charity_pct,
            self.donation.amount * (self.donation.charity_pct as f64 / 100.0),
            self.donation.cfp_pct,
            self.donation.amount * (self.donation.cfp_pct as f64 / 100.0),
            self.donation.author_pct,
            self.donation.amount * (self.donation.author_pct as f64 / 100.0),
        )
    }

    pub fn take_events(&mut self) -> Vec<DonationEvent> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn apply_event(&mut self, event: DonationEvent) {
        match event {
            DonationEvent::DonationCreated { .. } => {}
            DonationEvent::PaymentProcessed { transaction_id, .. } => {
                self.donation.status = DonationStatus::Completed;
                self.donation.transaction_id = Some(transaction_id);
            }
            DonationEvent::PaymentFailed { .. } => {
                self.donation.status = DonationStatus::Failed;
            }
            DonationEvent::DistributionProcessed { charity_amount, cfp_amount, author_amount, .. } => {
                if let Some(record) = self.distribution_history.last_mut() {
                    record.status = DistributionStatus::Processed;
                    record.charity_amount = charity_amount;
                    record.cfp_amount = cfp_amount;
                    record.author_amount = author_amount;
                }
            }
            DonationEvent::DistributionFailed { .. } => {
                if let Some(record) = self.distribution_history.last_mut() {
                    record.status = DistributionStatus::Failed;
                }
            }
            DonationEvent::Refunded { .. } => {
                self.donation.status = DonationStatus::Refunded;
                for record in &mut self.distribution_history {
                    if record.status == DistributionStatus::Processed {
                        record.status = DistributionStatus::Refunded;
                    }
                }
            }
        }
    }
}
