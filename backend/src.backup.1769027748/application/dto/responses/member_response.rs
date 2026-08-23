use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    enums::MemberStatus,
    value_objects::{Address, PhoneNumber, Email},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberResponse {
    pub member_id: Uuid,
    pub name: String,
    pub organization: String,
    pub pseudonym: Option<String>,
    pub primary_email: Email,
    pub recovery_email: Option<Email>,
    pub status: MemberStatus,
    pub join_date: DateTime<Utc>,
    pub address: Option<Address>,
    pub phone_numbers: Vec<PhoneNumber>,
    pub areas_of_interest: Vec<String>,
    pub download_limit: i32,
    pub downloads_used: i32,
    pub is_verified: bool,
    pub introduced_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorResponse {
    pub member_id: Uuid,
    pub orcid: String,
    pub bio: String,
    pub specialization: String,
    pub display_name: String,
    pub h_index: i32,
    pub total_downloads: i32,
    pub total_texts: i32,
    pub total_donations: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminResponse {
    pub admin_id: Uuid,
    pub member_id: Uuid,
    pub role: String,
    pub permissions: Vec<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratorResponse {
    pub mod_id: Uuid,
    pub member_id: Uuid,
    pub domain: String,
    pub expertise_areas: Vec<String>,
    pub approval_rate: f64,
    pub assigned_cases_count: i32,
    pub resolved_cases_count: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSummaryResponse {
    pub member_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: MemberStatus,
    pub join_date: DateTime<Utc>,
    pub total_donations: f64,
    pub total_texts: i32,
    pub total_comments: i32,
    pub total_votes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberSearchResponse {
    pub members: Vec<MemberResponse>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberStatsResponse {
    pub total_members: u64,
    pub active_members: u64,
    pub pending_members: u64,
    pub authors_count: u64,
    pub admins_count: u64,
    pub moderators_count: u64,
    pub new_members_today: u64,
    pub new_members_this_week: u64,
    pub new_members_this_month: u64,
    pub members_by_status: Vec<StatusCount>,
    pub members_by_role: Vec<RoleCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCount {
    pub status: String,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCount {
    pub role: String,
    pub count: u64,
}
