use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::{NotificationPriority, NotificationType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub notif_id: Uuid,
    pub member_id: Uuid,
    pub message: String,
    pub sent_date: DateTime<Utc>,
    pub notification_type: NotificationType,
    pub is_read: bool,
    pub priority: NotificationPriority,
    pub related_entity_id: Option<Uuid>,
    pub related_entity_type: Option<String>,
    pub action_url: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        member_id: Uuid,
        message: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
        related_entity_id: Option<Uuid>,
        related_entity_type: Option<String>,
        action_url: Option<String>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            notif_id: Uuid::new_v4(),
            member_id,
            message,
            sent_date: now,
            notification_type,
            is_read: false,
            priority,
            related_entity_id,
            related_entity_type,
            action_url,
            expires_at,
            created_at: now,
        }
    }

    pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }

    pub fn mark_as_unread(&mut self) {
        self.is_read = false;
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => Utc::now() > expires_at,
            None => false,
        }
    }

    pub fn is_urgent(&self) -> bool {
        matches!(self.priority, NotificationPriority::Urgent)
    }

    pub fn is_high_priority(&self) -> bool {
        matches!(self.priority, NotificationPriority::High) || self.is_urgent()
    }

    pub fn get_action_context(&self) -> Option<String> {
        match (&self.related_entity_type, &self.action_url) {
            (Some(entity_type), Some(url)) => Some(format!("{}:{}", entity_type, url)),
            (Some(entity_type), None) => Some(entity_type.clone()),
            (None, Some(url)) => Some(url.clone()),
            (None, None) => None,
        }
    }
}
