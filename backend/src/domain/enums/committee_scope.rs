#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitteeScope {
    Global,
    Regional,
    National,
    Local,
}

impl Default for CommitteeScope {
    fn default() -> Self {
        CommitteeScope::Local
    }
}

impl CommitteeScope {
    pub fn is_global(&self) -> bool {
        matches!(self, CommitteeScope::Global)
    }
}
