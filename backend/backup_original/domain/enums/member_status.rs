#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberStatus {
    Pending,
    Active,
    Suspended,
    Banned,
    Deleted,
}

impl Default for MemberStatus {
    fn default() -> Self {
        MemberStatus::Pending
    }
}

impl MemberStatus {
    pub fn can_login(&self) -> bool {
        matches!(self, MemberStatus::Active | MemberStatus::Pending)
    }
}
