#!/bin/bash
# fix_both_services.sh

echo "Fixing syntax errors in both service files..."

# Fix committee_service.rs
echo "Fixing committee_service.rs..."
cat > src/application/services/committee_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        models::{Committee, CommitteeMembership, PlagiarismCase},
        enums::{CommitteeScope, CommitteeStatus, CommitteeMembershipRole, CommitteeMembershipStatus},
    },
    application::dto::{
        requests::plagiarism_request::CreatePlagiarismCaseRequest,
        responses::committee_response::{
            CommitteeResponse,
            CommitteeMembershipResponse,
            CommitteeStatsResponse,
            CommitteeSearchResponse,
        },
    },
    infrastructure::database::repositories::{
        committee_repository::CommitteeRepository,
        RepositoryError,
    },
    utils::error::AppError,
};

#[async_trait]
pub trait CommitteeService: Send + Sync {
    async fn create_committee(&self, name: String, purpose: String, scope: CommitteeScope, chair_member_id: Option<Uuid>) -> Result<CommitteeResponse, AppError>;
    async fn get_committee(&self, committee_id: Uuid) -> Result<CommitteeResponse, AppError>;
    async fn update_committee(&self, committee_id: Uuid, name: Option<String>, purpose: Option<String>, status: Option<CommitteeStatus>) -> Result<CommitteeResponse, AppError>;
}
EOF

# Fix notification_service.rs
echo "Fixing notification_service.rs..."
cat > src/application/services/notification_service.rs << 'EOF'
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    domain::models::{Notification, Member},
    application::dto::{
        requests::notification_request::{
            CreateNotificationRequest,
            UpdateNotificationRequest,
            NotificationFilterRequest,
        },
        responses::notification_response::{
            NotificationResponse,
            NotificationListResponse,
            UnreadNotificationsResponse,
            NotificationStatsResponse,
        },
    },
    infrastructure::database::repositories::{
        notification_repository::NotificationRepository,
        member_repository::MemberRepository,
        RepositoryError,
    },
    utils::error::AppError,
};

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn create_notification(&self, request: CreateNotificationRequest) -> Result<NotificationResponse, AppError>;
    async fn get_notification(&self, notification_id: Uuid) -> Result<NotificationResponse, AppError>;
    async fn update_notification(&self, notification_id: Uuid, request: UpdateNotificationRequest) -> Result<NotificationResponse, AppError>;
    async fn delete_notification(&self, notification_id: Uuid) -> Result<(), AppError>;
    async fn mark_as_read(&self, notification_id: Uuid) -> Result<NotificationResponse, AppError>;
    async fn mark_all_as_read(&self, member_id: Uuid) -> Result<UnreadNotificationsResponse, AppError>;
    async fn get_unread_notifications(&self, member_id: Uuid) -> Result<NotificationListResponse, AppError>;
    async fn get_notifications(&self, member_id: Uuid, filter: Option<NotificationFilterRequest>) -> Result<NotificationListResponse, AppError>;
    async fn get_notification_stats(&self, member_id: Uuid) -> Result<NotificationStatsResponse, AppError>;
}
EOF

echo "Both files fixed! Running cargo check..."
cargo check
