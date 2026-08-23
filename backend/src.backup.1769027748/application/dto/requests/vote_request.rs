use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CastVoteRequest {
    pub case_id: String,
    
    #[validate(length(min = 1))]
    pub vote: String, // "plagiarized", "not_plagiarized", "abstain"
    
    #[validate(length(min = 10, max = 1000))]
    pub rationale: Option<String>,
    
    #[validate(range(min = 1, max = 10))]
    pub confidence_level: Option<i32>,
    
    pub evidence_references: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateVoteRequest {
    #[validate(length(min = 1))]
    pub vote: String,
    
    #[validate(length(min = 10, max = 1000))]
    pub rationale: Option<String>,
    
    #[validate(range(min = 1, max = 10))]
    pub confidence_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchVotesRequest {
    pub case_id: Option<String>,
    pub member_id: Option<String>,
    pub vote_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub has_rationale: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VoteStatisticsRequest {
    pub case_id: String,
    pub include_breakdown: Option<bool>,
    pub include_timeline: Option<bool>,
    pub include_voter_details: Option<bool>,
}
