use crate::application::dto::types::StatusCount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::TextStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResponse {
    pub text_id: Uuid,
    pub title: String,
    pub abstract_text: String,
    pub topic: String,
    pub keywords: Vec<String>,
    pub content: String,
    pub version: i32,
    pub status: TextStatus,
    pub upload_date: DateTime<Utc>,
    pub download_count: i32,
    pub total_donations: f64,
    pub avg_rating: f64,
    pub author_orcid: String,
    pub author_name: String,
    pub language: Option<String>,
    pub license_type: Option<String>,
    pub allow_comments: bool,
    pub allow_downloads: bool,
    pub allow_donations: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVersionResponse {
    pub version_id: Uuid,
    pub text_id: Uuid,
    pub version_number: i32,
    pub changes: String,
    pub change_summary: String,
    pub status: String,
    pub submitted_date: DateTime<Utc>,
    pub review_date: Option<DateTime<Utc>>,
    pub moderator_id: Option<Uuid>,
    pub moderator_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSummaryResponse {
    pub text_id: Uuid,
    pub title: String,
    pub abstract_text: String,
    pub topic: String,
    pub status: TextStatus,
    pub author_name: String,
    pub download_count: i32,
    pub total_donations: f64,
    pub avg_rating: f64,
    pub upload_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSearchResponse {
    pub texts: Vec<TextSummaryResponse>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStatsResponse {
    pub text_id: Uuid,
    pub title: String,
    pub total_downloads: i32,
    pub unique_downloaders: i32,
    pub total_donations: f64,
    pub donation_count: i32,
    pub avg_donation_amount: f64,
    pub total_comments: i32,
    pub avg_rating: f64,
    pub rating_count: i32,
    pub downloads_by_day: Vec<DownloadCount>,
    pub donations_by_day: Vec<DonationAmount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadCount {
    pub date: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationAmount {
    pub date: String,
    pub amount: f64,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAnalyticsResponse {
    pub total_texts: u64,
    pub published_texts: u64,
    pub draft_texts: u64,
    pub under_review_texts: u64,
    pub archived_texts: u64,
    pub total_downloads: u64,
    pub total_donations: f64,
    pub avg_text_rating: f64,
    pub top_downloaded_texts: Vec<TextSummaryResponse>,
    pub top_donated_texts: Vec<TextSummaryResponse>,
    pub top_rated_texts: Vec<TextSummaryResponse>,
    pub texts_by_topic: Vec<TopicCount>,
    pub texts_by_status: Vec<StatusCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicCount {
    pub topic: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResponse {
    pub download_id: Uuid,
    pub text_id: Uuid,
    pub text_title: String,
    pub download_date: DateTime<Utc>,
    pub download_type: String,
    pub file_size: Option<u64>,
    pub format: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub download_url: Option<String>,
}
