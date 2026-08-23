use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlagiarismStatus {
    Open,
    UnderReview,
    Voting,
    Closed,
    Appealed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlagiarismResolution {
    Plagiarized,
    NotPlagiarized,
    Appealed,
    Dismissed,
}

impl Default for PlagiarismStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl PlagiarismStatus {
    pub fn is_active_case(&self) -> bool {
        matches!(self, Self::Open | Self::UnderReview | Self::Voting | Self::Appealed)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }

    pub fn can_accept_votes(&self) -> bool {
        matches!(self, Self::Voting)
    }

    pub fn is_under_appeal(&self) -> bool {
        matches!(self, Self::Appealed)
    }

    pub fn next_status(&self) -> Option<Self> {
        match self {
            Self::Open => Some(Self::UnderReview),
            Self::UnderReview => Some(Self::Voting),
            Self::Voting => Some(Self::Closed),
            Self::Closed => None,
            Self::Appealed => Some(Self::UnderReview),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::UnderReview => "Under Review",
            Self::Voting => "Voting",
            Self::Closed => "Closed",
            Self::Appealed => "Appealed",
        }
    }
}

impl PlagiarismResolution {
    pub fn requires_blacklisting(&self) -> bool {
        matches!(self, Self::Plagiarized)
    }

    pub fn is_favorable_to_author(&self) -> bool {
        matches!(self, Self::NotPlagiarized)
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Plagiarized => "Plagiarized",
            Self::NotPlagiarized => "Not Plagiarized",
            Self::Appealed => "Appealed",
            Self::Dismissed => "Dismissed",
        }
    }
}
