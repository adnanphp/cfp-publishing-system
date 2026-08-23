use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::CommentStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub comment_id: Uuid,
    pub member_id: Uuid,
    pub text_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub date: DateTime<Utc>,
    pub is_public: bool,
    pub rating: Option<i32>,
    pub status: CommentStatus,
    pub like_count: i32,
    pub reply_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn new(
        member_id: Uuid,
        text_id: Uuid,
        content: String,
        parent_comment_id: Option<Uuid>,
        is_public: bool,
        rating: Option<i32>,
    ) -> Self {
        let now = Utc::now();
        Self {
            comment_id: Uuid::new_v4(),
            member_id,
            text_id,
            parent_comment_id,
            content,
            date: now,
            is_public,
            rating,
            status: CommentStatus::Active,
            like_count: 0,
            reply_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_root_comment(&self) -> bool {
        self.parent_comment_id.is_none()
    }

    pub fn is_reply(&self) -> bool {
        self.parent_comment_id.is_some()
    }

    pub fn increment_like_count(&mut self) {
        self.like_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn decrement_like_count(&mut self) {
        self.like_count = (self.like_count - 1).max(0);
        self.updated_at = Utc::now();
    }

    pub fn increment_reply_count(&mut self) {
        self.reply_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn flag(&mut self) {
        self.status = CommentStatus::Flagged;
        self.updated_at = Utc::now();
    }

    pub fn remove(&mut self) {
        self.status = CommentStatus::Removed;
        self.updated_at = Utc::now();
    }

    pub fn restore(&mut self) {
        self.status = CommentStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, CommentStatus::Active)
    }

    pub fn is_flagged(&self) -> bool {
        matches!(self.status, CommentStatus::Flagged)
    }

    pub fn is_removed(&self) -> bool {
        matches!(self.status, CommentStatus::Removed)
    }

    pub fn validate_rating(&self) -> bool {
        match self.rating {
            Some(rating) => rating >= 1 && rating <= 5,
            None => true,
        }
    }
}
