use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePlagiarismCaseRequest {
    pub text_id: String,
    pub committee_id: String,
    
    #[validate(length(min = 10, max = 2000))]
    pub description: String,
    
    pub evidence: Vec<PlagiarismEvidence>,
    pub similarity_score: Option<f64>,
    pub suspected_source: Option<String>,
    pub priority: Option<String>, // "low", "medium", "high", "urgent"
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PlagiarismEvidence {
    pub source_text: String,
    pub matched_text: String,
    pub similarity_percentage: f64,
    pub source_url: Option<String>,
    pub source_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePlagiarismCaseRequest {
    pub status: Option<String>, // "open", "under_review", "voting", "closed", "appealed"
    pub resolution: Option<String>, // "plagiarized", "not_plagiarized", "appealed", "dismissed"
    pub description: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StartVotingRequest {
    pub voting_period_days: Option<i32>,
    pub required_votes: Option<i32>,
    pub quorum_percentage: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AppealPlagiarismCaseRequest {
    #[validate(length(min = 10, max = 2000))]
    pub reason: String,
    
    pub new_evidence: Option<Vec<PlagiarismEvidence>>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchPlagiarismCasesRequest {
    pub text_id: Option<String>,
    pub committee_id: Option<String>,
    pub status: Option<String>,
    pub resolution: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub priority: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AssignModeratorRequest {
    pub moderator_id: String,
    pub case_id: String,
    pub role: Option<String>, // "primary", "secondary", "reviewer"
}
