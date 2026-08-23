// backend/src/domain/enums/vote_type.rs
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "snake_case")]
pub enum VoteType {
    Plagiarized,
    NotPlagiarized,
    Abstain,
}

impl std::fmt::Display for VoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plagiarized => write!(f, "plagiarized"),
            Self::NotPlagiarized => write!(f, "not_plagiarized"),
            Self::Abstain => write!(f, "abstain"),
        }
    }
}
