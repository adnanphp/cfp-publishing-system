use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeResponse {
    pub id: String,
    pub name: String,
    // Add other fields as needed
}
