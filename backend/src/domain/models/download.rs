use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub download_id: u32,
    pub member_id: u32,
    pub text_id: u32,
    pub download_date: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub country: Option<String>,
}

impl Download {
    pub fn new(
        member_id: u32,
        text_id: u32,
        ip_address: Option<String>,
        user_agent: Option<String>,
        country: Option<String>,
    ) -> Self {
        Self {
            download_id: 0, // Will be set by database
            member_id,
            text_id,
            download_date: Utc::now(),
            ip_address,
            user_agent,
            country,
        }
    }
    
    pub fn anonymize_ip(&self) -> Option<String> {
        self.ip_address.as_ref().map(|ip| {
            let parts: Vec<&str> = ip.split('.').collect();
            if parts.len() == 4 {
                format!("{}.{}.{}.xxx", parts[0], parts[1], parts[2])
            } else {
                "unknown".to_string()
            }
        })
    }
    
    pub fn get_browser_info(&self) -> Option<String> {
        self.user_agent.as_ref().and_then(|ua| {
            if ua.contains("Chrome") {
                Some("Chrome".to_string())
            } else if ua.contains("Firefox") {
                Some("Firefox".to_string())
            } else if ua.contains("Safari") && !ua.contains("Chrome") {
                Some("Safari".to_string())
            } else if ua.contains("Edge") {
                Some("Edge".to_string())
            } else if ua.contains("Opera") {
                Some("Opera".to_string())
            } else {
                None
            }
        })
    }
}
