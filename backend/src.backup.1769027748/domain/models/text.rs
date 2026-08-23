// backend/src/domain/models/text.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::enums::TextStatus;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Text {
    pub text_id: i32,
    
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(max = 5000))]
    pub abstract_text: Option<String>,
    
    #[validate(length(max = 100))]
    pub topic: Option<String>,
    
    // Multivalued attribute
    pub keywords: Vec<String>,
    
    pub author_orcid: String,
    pub version: i32,
    
    pub upload_date: DateTime<Utc>,
    pub status: TextStatus,
    
    // File information
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    
    // Derived attributes
    pub download_count: i64,
    pub total_donations: f64,
    pub avg_rating: Option<f64>,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Text {
    pub fn new(
        title: String,
        abstract_text: Option<String>,
        topic: Option<String>,
        keywords: Vec<String>,
        author_orcid: String,
        file_path: Option<String>,
        file_size: Option<i64>,
        file_hash: Option<String>,
    ) -> Result<Self, String> {
        let now = Utc::now();
        
        let text = Self {
            text_id: 0,
            title,
            abstract_text,
            topic,
            keywords,
            author_orcid,
            version: 1,
            upload_date: now,
            status: TextStatus::Draft,
            file_path,
            file_size,
            file_hash,
            download_count: 0,
            total_donations: 0.0,
            avg_rating: None,
            created_at: now,
            updated_at: now,
        };
        
        text.validate()
            .map_err(|e| format!("Invalid text data: {}", e))?;
        
        // Validate keywords count
        if text.keywords.len() > 10 {
            return Err("Maximum 10 keywords allowed".to_string());
        }
        
        Ok(text)
    }
    
    pub fn increment_download(&mut self) {
        self.download_count += 1;
        self.updated_at = Utc::now();
    }
    
    pub fn add_donation(&mut self, amount: f64) {
        self.total_donations += amount;
        self.updated_at = Utc::now();
    }
    
    pub fn update_rating(&mut self, new_rating: f64, total_ratings: i32) {
        if total_ratings > 0 {
            self.avg_rating = Some(new_rating);
        }
        self.updated_at = Utc::now();
    }
    
    pub fn can_be_downloaded(&self) -> bool {
        matches!(self.status, TextStatus::Published)
    }
    
    pub fn publish(&mut self) {
        self.status = TextStatus::Published;
        self.updated_at = Utc::now();
    }
    
    pub fn archive(&mut self) {
        self.status = TextStatus::Archived;
        self.updated_at = Utc::now();
    }
    
    pub fn create_new_version(&self, changes: String, change_summary: String) -> (Self, String) {
        let mut new_version = self.clone();
        new_version.text_id = 0;
        new_version.version += 1;
        new_version.status = TextStatus::Draft;
        new_version.upload_date = Utc::now();
        new_version.created_at = Utc::now();
        new_version.updated_at = Utc::now();
        
        (new_version, changes)
    }
}
