// backend/src/domain/models/author.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Author {
    #[validate(length(min = 16, max = 20))]
    pub orcid: String,
    
    pub member_id: i32,
    
    #[validate(length(max = 2000))]
    pub bio: Option<String>,
    
    #[validate(length(max = 100))]
    pub specialization: Option<String>,
    
    // Derived attributes
    pub h_index: Option<i32>,
    pub total_downloads: i64,
    pub total_donations: f64,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Author {
    pub fn new(orcid: String, member_id: i32, bio: Option<String>, specialization: Option<String>) -> Result<Self, String> {
        let author = Self {
            orcid,
            member_id,
            bio,
            specialization,
            h_index: None,
            total_downloads: 0,
            total_donations: 0.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        author.validate()
            .map_err(|e| format!("Invalid author data: {}", e))?;
        
        Ok(author)
    }
    
    pub fn update_statistics(&mut self, downloads: i64, donations: f64) {
        self.total_downloads = downloads;
        self.total_donations = donations;
        self.updated_at = Utc::now();
    }
    
    pub fn calculate_h_index(&self, citations: Vec<i32>) -> i32 {
        let mut sorted = citations.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        
        for (i, &citations) in sorted.iter().enumerate() {
            if citations < (i + 1) as i32 {
                return i as i32;
            }
        }
        
        sorted.len() as i32
    }
}
