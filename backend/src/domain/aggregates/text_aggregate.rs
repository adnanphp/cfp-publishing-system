use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::{
    models::{Text, TextVersion, Comment, Download, Donation},
    enums::TextStatus,
    value_objects::PercentageDistribution,
    events::text_events::TextEvent,
};

#[derive(Debug, Clone)]
pub struct TextAggregate {
    pub text: Text,
    pub versions: Vec<TextVersion>,
    pub comments: Vec<Comment>,
    pub downloads: Vec<Download>,
    pub donations: Vec<Donation>,
    pub current_version: Option<TextVersion>,
    pub pending_events: Vec<TextEvent>,
}

impl TextAggregate {
    pub fn new(
        title: String,
        abstract_text: String,
        topic: String,
        keywords: Vec<String>,
        author_orcid: String,
        author_member_id: Uuid,
    ) -> Self {
        let text = Text::new(
            title,
            abstract_text,
            topic,
            keywords,
            author_orcid,
            author_member_id,
        );

        Self {
            text,
            versions: Vec::new(),
            comments: Vec::new(),
            downloads: Vec::new(),
            donations: Vec::new(),
            current_version: None,
            pending_events: Vec::new(),
        }
    }

    pub fn text_id(&self) -> Uuid {
        self.text.text_id
    }

    pub fn create_new_version(&mut self, changes: String, change_summary: String) {
        let version_number = (self.versions.len() + 1) as i32;
        let version = TextVersion::new(
            self.text_id(),
            changes,
            change_summary,
            version_number,
        );
        
        self.versions.push(version.clone());
        self.current_version = Some(version);
        
        self.pending_events.push(TextEvent::VersionCreated {
            text_id: self.text_id(),
            version_number,
            timestamp: Utc::now(),
        });
    }

    pub fn approve_version(&mut self, version_id: Uuid, moderator_id: Uuid) {
        if let Some(version) = self.versions.iter_mut().find(|v| v.version_id == version_id) {
            version.approve(moderator_id);
            
            if version.is_approved() {
                self.text.status = TextStatus::Published;
                self.current_version = Some(version.clone());
                
                self.pending_events.push(TextEvent::VersionApproved {
                    text_id: self.text_id(),
                    version_id,
                    moderator_id,
                    timestamp: Utc::now(),
                });
            }
        }
    }

    pub fn reject_version(&mut self, version_id: Uuid, moderator_id: Uuid) {
        if let Some(version) = self.versions.iter_mut().find(|v| v.version_id == version_id) {
            version.reject(moderator_id);
            
            self.pending_events.push(TextEvent::VersionRejected {
                text_id: self.text_id(),
                version_id,
                moderator_id,
                timestamp: Utc::now(),
            });
        }
    }

    pub fn submit_for_review(&mut self) {
        if self.text.status == TextStatus::Draft {
            self.text.status = TextStatus::UnderReview;
            
            self.pending_events.push(TextEvent::SubmittedForReview {
                text_id: self.text_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn publish(&mut self) {
        if self.text.status != TextStatus::Published {
            self.text.status = TextStatus::Published;
            
            self.pending_events.push(TextEvent::Published {
                text_id: self.text_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn archive(&mut self) {
        if self.text.status != TextStatus::Archived {
            self.text.status = TextStatus::Archived;
            
            self.pending_events.push(TextEvent::Archived {
                text_id: self.text_id(),
                timestamp: Utc::now(),
            });
        }
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    pub fn add_download(&mut self, download: Download) {
        self.downloads.push(download);
        self.text.download_count += 1;
    }

    pub fn add_donation(&mut self, donation: Donation) {
        self.donations.push(donation.clone());
        self.text.total_donations += donation.amount;
        
        // Recalculate average rating if donation has rating
        if let Some(rating) = donation.rating {
            self.update_average_rating(rating);
        }
    }

    fn update_average_rating(&mut self, new_rating: i32) {
        let total_donations_with_rating = self.donations.iter()
            .filter(|d| d.rating.is_some())
            .count();
        
        if total_donations_with_rating > 0 {
            let sum_ratings: i32 = self.donations.iter()
                .filter_map(|d| d.rating)
                .sum();
            
            self.text.avg_rating = sum_ratings as f64 / total_donations_with_rating as f64;
        }
    }

    pub fn get_comment(&self, comment_id: Uuid) -> Option<&Comment> {
        self.comments.iter().find(|c| c.comment_id == comment_id)
    }

    pub fn get_version(&self, version_id: Uuid) -> Option<&TextVersion> {
        self.versions.iter().find(|v| v.version_id == version_id)
    }

    pub fn is_published(&self) -> bool {
        self.text.status == TextStatus::Published
    }

    pub fn is_draft(&self) -> bool {
        self.text.status == TextStatus::Draft
    }

    pub fn is_under_review(&self) -> bool {
        self.text.status == TextStatus::UnderReview
    }

    pub fn can_be_downloaded(&self) -> bool {
        self.is_published() || self.is_under_review()
    }

    pub fn calculate_total_revenue(&self) -> f64 {
        self.donations.iter().map(|d| d.amount).sum()
    }

    pub fn calculate_charity_distribution(&self) -> PercentageDistribution {
        let mut charity_total = 0.0;
        let mut cfp_total = 0.0;
        let mut author_total = 0.0;
        
        for donation in &self.donations {
            charity_total += donation.amount * (donation.charity_pct as f64 / 100.0);
            cfp_total += donation.amount * (donation.cfp_pct as f64 / 100.0);
            author_total += donation.amount * (donation.author_pct as f64 / 100.0);
        }
        
        let total = charity_total + cfp_total + author_total;
        
        if total > 0.0 {
            let charity_pct = (charity_total / total * 100.0).round() as i32;
            let cfp_pct = (cfp_total / total * 100.0).round() as i32;
            let author_pct = (author_total / total * 100.0).round() as i32;
            
            PercentageDistribution::new(charity_pct, cfp_pct, author_pct).unwrap_or_default()
        } else {
            PercentageDistribution::default()
        }
    }

    pub fn take_events(&mut self) -> Vec<TextEvent> {
        std::mem::take(&mut self.pending_events)
    }

    pub fn apply_event(&mut self, event: TextEvent) {
        match event {
            TextEvent::TextCreated { .. } => {}
            TextEvent::SubmittedForReview { .. } => {
                self.text.status = TextStatus::UnderReview;
            }
            TextEvent::Published { .. } => {
                self.text.status = TextStatus::Published;
            }
            TextEvent::Archived { .. } => {
                self.text.status = TextStatus::Archived;
            }
            TextEvent::VersionCreated { version_number, .. } => {
                // Version already added in create_new_version method
            }
            TextEvent::VersionApproved { version_id, moderator_id, .. } => {
                if let Some(version) = self.versions.iter_mut().find(|v| v.version_id == version_id) {
                    version.approve(moderator_id);
                    self.text.status = TextStatus::Published;
                }
            }
            TextEvent::VersionRejected { version_id, moderator_id, .. } => {
                if let Some(version) = self.versions.iter_mut().find(|v| v.version_id == version_id) {
                    version.reject(moderator_id);
                }
            }
            TextEvent::Downloaded { .. } => {
                self.text.download_count += 1;
            }
            TextEvent::Donated { amount, .. } => {
                self.text.total_donations += amount;
            }
        }
    }
}
