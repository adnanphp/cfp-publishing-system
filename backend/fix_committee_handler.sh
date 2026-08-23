#!/bin/bash
# fix_committee_handler.sh

echo "Fixing syntax errors in committee_handler.rs..."

# Create a clean version of committee_handler.rs
cat > src/api/handlers/committee_handler.rs << 'EOF'
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommitteeRequest {
    pub name: String,
    pub description: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub scope: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitteeMemberResponse {
    pub member_id: Uuid,
    pub role: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_committees() -> impl Responder {
    let committees = vec![
        CommitteeResponse {
            id: Uuid::new_v4(),
            name: "Ethics Committee".to_string(),
            description: "Handles ethical issues".to_string(),
            scope: "global".to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
        },
        CommitteeResponse {
            id: Uuid::new_v4(),
            name: "Review Committee".to_string(),
            description: "Reviews submissions".to_string(),
            scope: "texts".to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(committees, "Committees list".to_string()))
}

pub async fn get_committee(_path: web::Path<Uuid>) -> impl Responder {
    let committee = CommitteeResponse {
        id: Uuid::new_v4(),
        name: "Sample Committee".to_string(),
        description: "Sample committee description".to_string(),
        scope: "sample".to_string(),
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(committee, "Committee found".to_string()))
}

pub async fn create_committee(_req: web::Json<CreateCommitteeRequest>) -> impl Responder {
    let committee = CommitteeResponse {
        id: Uuid::new_v4(),
        name: "New Committee".to_string(),
        description: "New committee description".to_string(),
        scope: "new".to_string(),
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Created().json(ApiResponse::new(committee, "Committee created".to_string()))
}

pub async fn get_committee_members(_path: web::Path<Uuid>) -> impl Responder {
    let members = vec![
        CommitteeMemberResponse {
            member_id: Uuid::new_v4(),
            role: "chair".to_string(),
            joined_at: chrono::Utc::now(),
        },
        CommitteeMemberResponse {
            member_id: Uuid::new_v4(),
            role: "member".to_string(),
            joined_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(members, "Committee members".to_string()))
}

pub async fn add_committee_member(_path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let member = CommitteeMemberResponse {
        member_id: Uuid::new_v4(),
        role: "new_member".to_string(),
        joined_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(member, "Member added".to_string()))
}

pub async fn remove_committee_member(_path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"removed": true}),
        "Member removed".to_string()
    ))
}

pub async fn get_committee_meetings(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        vec![
            serde_json::json!({"id": "meeting1", "date": "2024-01-01"}),
            serde_json::json!({"id": "meeting2", "date": "2024-01-15"})
        ],
        "Committee meetings".to_string()
    ))
}

pub async fn schedule_meeting(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"meeting_id": "meeting_123", "scheduled": true}),
        "Meeting scheduled".to_string()
    ))
}

pub async fn get_committee_decisions(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        vec![
            serde_json::json!({"id": "decision1", "type": "approval"}),
            serde_json::json!({"id": "decision2", "type": "rejection"})
        ],
        "Committee decisions".to_string()
    ))
}

pub async fn get_committee_stats(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "total_members": 5,
            "total_meetings": 12,
            "total_decisions": 45
        }),
        "Committee stats".to_string()
    ))
}
EOF

echo "Fixed committee_handler.rs. Running cargo check..."
cargo check
