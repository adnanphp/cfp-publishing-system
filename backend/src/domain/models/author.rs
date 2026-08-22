use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub orcid: String,
    pub member_id: Option<u32>,
    pub bio: Option<String>,
    pub specialization: Option<String>,
    pub h_index: i32,
    pub total_downloads: i32,
}
