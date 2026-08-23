// backend/src/domain/models/plagiarism_case.rs
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::enums::PlagiarismStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PlagiarismCase {
    // Weak entity composite key
    pub case_id: i32,
    pub committee_id: i32,
    
    pub text_id: i32,
    
    pub opened_date: NaiveDate,
    pub description: Option<String>,
    pub status: PlagiarismStatus,
    pub resolution: Option<String>, // 'plagiarized', 'not_plagiarized', 'appealed'
    pub closed_date: Option<NaiveDate>,
    
    // Derived statistics
    pub total_votes: i32,
    pub plagiarized_votes: i32,
    pub not_plagiarized_votes: i32,
    pub abstain_votes: i32,
    
    // Business rule: 2/3 majority required
    pub required_majority: i32,
    pub has_majority: bool,
    
    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PlagiarismCase {
    pub fn new(
        committee_id: i32,
        text_id: i32,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            case_id: 0,
            committee_id,
            text_id,
            opened_date: now.date_naive(),
            description,
            status: PlagiarismStatus::Open,
            resolution: None,
            closed_date: None,
            total_votes: 0,
            plagiarized_votes: 0,
            not_plagiarized_votes: 0,
            abstain_votes: 0,
            required_majority: 0,
            has_majority: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_vote(&mut self, vote_type: &str) {
        self.total_votes += 1;
        
        match vote_type {
            "plagiarized" => self.plagiarized_votes += 1,
            "not_plagiarized" => self.not_plagiarized_votes += 1,
            "abstain" => self.abstain_votes += 1,
            _ => {}
        }
        
        self.update_majority();
        self.updated_at = Utc::now();
    }
    
    fn update_majority(&mut self) {
        // Business rule: 2/3 majority required
        self.required_majority = (self.total_votes as f32 * 2.0 / 3.0).ceil() as i32;
        
        self.has_majority = self.plagiarized_votes >= self.required_majority
            || self.not_plagiarized_votes >= self.required_majority;
        
        // If majority reached, update status
        if self.has_majority {
            if self.plagiarized_votes > self.not_plagiarized_votes {
                self.resolution = Some("plagiarized".to_string());
                self.status = PlagiarismStatus::Closed;
            } else {
                self.resolution = Some("not_plagiarized".to_string());
                self.status = PlagiarismStatus::Closed;
            }
            self.closed_date = Some(Utc::now().date_naive());
        }
    }
    
    pub fn can_vote(&self) -> bool {
        matches!(self.status, PlagiarismStatus::Voting)
    }
    
    pub fn is_voting_period_active(&self) -> bool {
        let voting_end = self.opened_date + chrono::Duration::days(14);
        Utc::now().date_naive() <= voting_end
    }
    
    pub fn move_to_voting(&mut self) {
        self.status = PlagiarismStatus::Voting;
        self.updated_at = Utc::now();
    }
    
    pub fn close(&mut self, resolution: String) {
        self.status = PlagiarismStatus::Closed;
        self.resolution = Some(resolution);
        self.closed_date = Some(Utc::now().date_naive());
        self.updated_at = Utc::now();
    }
    
    pub fn appeal(&mut self) {
        self.status = PlagiarismStatus::Appealed;
        self.resolution = Some("appealed".to_string());
        self.updated_at = Utc::now();
    }
    
    // Business rule: 3 violations = automatic blacklist
    pub fn is_third_violation(&self, previous_violations: i32) -> bool {
        previous_violations >= 2 // This would be the 3rd
    }
}
