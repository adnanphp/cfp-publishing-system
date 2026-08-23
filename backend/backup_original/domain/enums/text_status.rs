#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Published,
    Rejected,
    Archived,
}

impl Default for TextStatus {
    fn default() -> Self {
        TextStatus::Draft
    }
}

impl TextStatus {
    pub fn can_be_edited(&self) -> bool {
        matches!(self, TextStatus::Draft | TextStatus::Rejected)
    }
}
