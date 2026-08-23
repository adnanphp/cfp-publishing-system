use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    System,
    Donation,
    Comment,
    Plagiarism,
    Committee,
    Message,
    TextUpdate,
    Member,
    Security,
}

impl NotificationType {
    pub fn default_priority(&self) -> crate::domain::enums::NotificationPriority {
        match self {
            Self::System => crate::domain::enums::NotificationPriority::High,
            Self::Security => crate::domain::enums::NotificationPriority::Urgent,
            Self::Plagiarism => crate::domain::enums::NotificationPriority::High,
            Self::Donation => crate::domain::enums::NotificationPriority::Medium,
            Self::Committee => crate::domain::enums::NotificationPriority::Medium,
            Self::Comment => crate::domain::enums::NotificationPriority::Low,
            Self::Message => crate::domain::enums::NotificationPriority::Medium,
            Self::TextUpdate => crate::domain::enums::NotificationPriority::Low,
            Self::Member => crate::domain::enums::NotificationPriority::Medium,
        }
    }

    pub fn requires_acknowledgment(&self) -> bool {
        matches!(self, Self::System | Self::Security | Self::Plagiarism)
    }

    pub fn should_send_email(&self) -> bool {
        matches!(
            self,
            Self::System | Self::Security | Self::Donation | Self::Committee | Self::Plagiarism
        )
    }

    pub fn expiration_days(&self) -> Option<i32> {
        match self {
            Self::System => Some(30),
            Self::Security => Some(7),
            Self::Donation => Some(90),
            Self::Comment => Some(30),
            Self::Plagiarism => Some(60),
            Self::Committee => Some(30),
            Self::Message => Some(30),
            Self::TextUpdate => Some(7),
            Self::Member => Some(30),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Donation => "Donation",
            Self::Comment => "Comment",
            Self::Plagiarism => "Plagiarism",
            Self::Committee => "Committee",
            Self::Message => "Message",
            Self::TextUpdate => "Text Update",
            Self::Member => "Member",
            Self::Security => "Security",
        }
    }
}
