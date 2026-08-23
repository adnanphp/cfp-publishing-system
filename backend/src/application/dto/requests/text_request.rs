use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTextRequest {
    #[validate(length(min = 5, max = 255))]
    pub title: String,
    
    #[validate(length(min = 50, max = 2000))]
    pub abstract_text: String,
    
    #[validate(length(min = 2, max = 100))]
    pub topic: String,
    
    pub keywords: Vec<String>,
    pub content: String,
    pub language: Option<String>,
    pub license_type: Option<String>,
    pub allow_comments: Option<bool>,
    pub allow_downloads: Option<bool>,
    pub allow_donations: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTextRequest {
    #[validate(length(min = 5, max = 255))]
    pub title: Option<String>,
    
    #[validate(length(min = 50, max = 2000))]
    pub abstract_text: Option<String>,
    
    #[validate(length(min = 2, max = 100))]
    pub topic: Option<String>,
    
    pub keywords: Option<Vec<String>>,
    pub content: Option<String>,
    pub status: Option<String>, // "draft", "under_review", "published", "archived"
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTextVersionRequest {
    #[validate(length(min = 10))]
    pub changes: String,
    
    #[validate(length(min = 10, max = 500))]
    pub change_summary: String,
    
    pub is_major_version: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReviewTextVersionRequest {
    pub approved: bool,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SearchTextsRequest {
    pub query: Option<String>,
    pub author_orcid: Option<String>,
    pub topic: Option<String>,
    pub status: Option<String>,
    pub min_rating: Option<f64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DownloadTextRequest {
    pub download_type: String, // "full_text", "abstract", "metadata"
    pub format: Option<String>, // "pdf", "epub", "txt"
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReportTextRequest {
    pub reason: String,
    pub description: String,
    pub category: String, // "plagiarism", "inappropriate", "copyright", "other"
}
