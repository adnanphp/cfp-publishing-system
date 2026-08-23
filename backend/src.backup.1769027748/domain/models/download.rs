use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub download_id: Uuid,
    pub member_id: Uuid,
    pub text_id: Uuid,
    pub download_date: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub country: Option<String>,
    pub download_type: DownloadType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadType {
    FullText,
    Abstract,
    Metadata,
}

impl Download {
    pub fn new(
        member_id: Uuid,
        text_id: Uuid,
        ip_address: String,
        user_agent: Option<String>,
        country: Option<String>,
        download_type: DownloadType,
    ) -> Self {
        Self {
            download_id: Uuid::new_v4(),
            member_id,
            text_id,
            download_date: Utc::now(),
            ip_address,
            user_agent,
            country,
            download_type,
        }
    }

    pub fn is_full_text_download(&self) -> bool {
        matches!(self.download_type, DownloadType::FullText)
    }

    pub fn is_abstract_download(&self) -> bool {
        matches!(self.download_type, DownloadType::Abstract)
    }

    pub fn get_location_info(&self) -> String {
        match &self.country {
            Some(country) => country.clone(),
            None => "Unknown".to_string(),
        }
    }
}
