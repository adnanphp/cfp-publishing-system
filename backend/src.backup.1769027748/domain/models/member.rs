use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub member_id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub pseudonym: Option<String>,
    pub primary_email: String,
    pub recovery_email: Option<String>,
    pub password_hash: String,
    pub verification_matrix: Option<String>,
    pub matrix_expiry: Option<DateTime<Utc>>,
    pub join_date: DateTime<Utc>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub introduced_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
