use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub text_id: u32,
    pub author_orcid: Option<String>,
    pub title: String,
    pub abstract_text: Option<String>,
    pub topic: Option<String>,
    pub version: u32,
    pub upload_date: String,
    pub status: String,
    pub download_count: u32,
    pub total_donations: f64,
    pub avg_rating: f64,
}

impl Text {
    pub fn new(
        text_id: u32,
        author_orcid: Option<String>,
        title: String,
        status: String,
    ) -> Self {
        Self {
            text_id,
            author_orcid,
            title,
            abstract_text: None,
            topic: None,
            version: 1,
            upload_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            status,
            download_count: 0,
            total_donations: 0.0,
            avg_rating: 0.0,
        }
    }
}
