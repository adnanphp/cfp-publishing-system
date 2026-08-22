use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub member_id: u32,
    pub name: String,
    pub primary_email: String,
    pub organization: Option<String>,
}

impl Member {
    pub fn new(member_id: u32, name: String, primary_email: String) -> Self {
        Self {
            member_id,
            name,
            primary_email,
            organization: None,
        }
    }
}
