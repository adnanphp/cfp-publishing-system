use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteeScope {
    Plagiarism,
    Content,
    Finance,
    Appeals,
}

impl CommitteeScope {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Plagiarism => "Handles plagiarism detection and resolution cases",
            Self::Content => "Manages content moderation and approval processes",
            Self::Finance => "Oversees financial matters and donation distributions",
            Self::Appeals => "Processes appeals and dispute resolutions",
        }
    }

    pub fn required_expertise(&self) -> Vec<&'static str> {
        match self {
            Self::Plagiarism => vec!["Plagiarism Detection", "Academic Integrity", "Legal Knowledge"],
            Self::Content => vec!["Content Moderation", "Subject Expertise", "Quality Assurance"],
            Self::Finance => vec!["Financial Management", "Accounting", "Regulatory Compliance"],
            Self::Appeals => vec!["Conflict Resolution", "Legal Framework", "Ethical Standards"],
        }
    }

    pub fn voting_quorum_percentage(&self) -> i32 {
        match self {
            Self::Plagiarism => 67, // 2/3 majority for plagiarism cases
            Self::Content => 51,    // Simple majority for content decisions
            Self::Finance => 75,    // Higher threshold for financial decisions
            Self::Appeals => 60,    // Supermajority for appeals
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Plagiarism => "Plagiarism",
            Self::Content => "Content",
            Self::Finance => "Finance",
            Self::Appeals => "Appeals",
        }
    }
}
