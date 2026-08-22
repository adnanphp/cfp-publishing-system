#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlagiarismStatus {
    Reported,
    UnderInvestigation,
    Confirmed,
    Dismissed,
    Resolved,
}

impl Default for PlagiarismStatus {
    fn default() -> Self {
        PlagiarismStatus::Reported
    }
}

impl PlagiarismStatus {
    pub fn is_open_case(&self) -> bool {
        matches!(self, 
            PlagiarismStatus::Reported | 
            PlagiarismStatus::UnderInvestigation
        )
    }
}
