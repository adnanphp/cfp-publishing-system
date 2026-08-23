use serde::{Serialize, Deserialize};
pub mod charity_status;
pub mod committee_scope;
pub mod plagiarism_status;
pub mod notification_type;
pub mod notification_priority;
pub mod role_enum;

// Re-export for convenience
pub use charity_status::*;
pub use committee_scope::*;
pub use plagiarism_status::*;
pub use notification_type::*;
pub use notification_priority::*;
pub use role_enum::*;

// Additional enums that might be referenced elsewhere
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemberStatus {
    Pending,
    Active,
    Suspended,
    Banned,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStatus {
    Draft,
    UnderReview,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DonationStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    Plagiarized,
    NotPlagiarized,
    Abstain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentStatus {
    Active,
    Flagged,
    Removed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteeStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteeMembershipRole {
    Chair,
    Member,
    Secretary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteeMembershipStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionStatus {
    Pending,
    Approved,
    Rejected,
}
