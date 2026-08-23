use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for NotificationPriority {
    fn default() -> Self {
        Self::Medium
    }
}

impl NotificationPriority {
    pub fn delivery_timeout_seconds(&self) -> u64 {
        match self {
            Self::Urgent => 60,    // 1 minute
            Self::High => 300,     // 5 minutes
            Self::Medium => 3600,  // 1 hour
            Self::Low => 86400,    // 24 hours
        }
    }

    pub fn retry_attempts(&self) -> i32 {
        match self {
            Self::Urgent => 5,
            Self::High => 3,
            Self::Medium => 2,
            Self::Low => 1,
        }
    }

    pub fn should_push_notify(&self) -> bool {
        matches!(self, Self::Urgent | Self::High)
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Urgent => "#FF0000", // Red
            Self::High => "#FFA500",   // Orange
            Self::Medium => "#FFFF00", // Yellow
            Self::Low => "#00FF00",    // Green
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "urgent" => Some(Self::Urgent),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }
}
