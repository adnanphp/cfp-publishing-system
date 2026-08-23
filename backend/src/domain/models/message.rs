use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub subject: String,
    pub body: String,
    pub sent_at: DateTime<Utc>,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub parent_message_id: Option<Uuid>,
    pub message_type: MessageType,
    pub priority: MessagePriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    DirectMessage,
    SystemAnnouncement,
    CommitteeCommunication,
    DonationReceipt,
    PlagiarismAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Message {
    pub fn new(
        sender_id: Uuid,
        recipient_id: Uuid,
        subject: String,
        body: String,
        message_type: MessageType,
        priority: MessagePriority,
        parent_message_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            message_id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            subject,
            body,
            sent_at: now,
            is_read: false,
            read_at: None,
            parent_message_id,
            message_type,
            priority,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_as_read(&mut self) {
        if !self.is_read {
            self.is_read = true;
            self.read_at = Some(Utc::now());
            self.updated_at = Utc::now();
        }
    }

    pub fn mark_as_unread(&mut self) {
        self.is_read = false;
        self.read_at = None;
        self.updated_at = Utc::now();
    }

    pub fn is_direct_message(&self) -> bool {
        matches!(self.message_type, MessageType::DirectMessage)
    }

    pub fn is_system_message(&self) -> bool {
        matches!(self.message_type, MessageType::SystemAnnouncement)
    }

    pub fn is_reply(&self) -> bool {
        self.parent_message_id.is_some()
    }

    pub fn is_urgent(&self) -> bool {
        matches!(self.priority, MessagePriority::Urgent)
    }

    pub fn reply(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        body: String,
        priority: MessagePriority,
    ) -> Self {
        let subject = if self.subject.starts_with("Re: ") {
            self.subject.clone()
        } else {
            format!("Re: {}", self.subject)
        };

        Self::new(
            sender_id,
            recipient_id,
            subject,
            body,
            MessageType::DirectMessage,
            priority,
            Some(self.message_id),
        )
    }
}
