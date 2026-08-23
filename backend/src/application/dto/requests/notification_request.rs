use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationRequest {
    pub title: String,
    pub message: String,
    // Add other fields as needed
}
