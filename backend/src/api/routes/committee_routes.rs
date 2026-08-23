use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommitteeRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 10, max = 2000))]
    pub purpose: String,
    
    #[validate(length(min = 1, max = 50))]
    pub scope: String, // "plagiarism", "content", "finance", "appeals"
    
    pub formation_date: Option<chrono::NaiveDate>,
    
    pub initial_members: Option<Vec<CommitteeMember>>,
    
    pub rules: Option<serde_json::Value>,
    
    pub term_duration_months: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CommitteeMember {
    #[validate(range(min = 1))]
    pub member_id: i32,
    
    #[validate(length(min = 1, max = 50))]
    pub role: String, // "chair", "member", "secretary"
    
    pub term_end_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddCommitteeMemberRequest {
    #[validate(range(min = 1))]
    pub member_id: i32,
    
    #[validate(length(min = 1, max = 50))]
    pub role: String,
    
    pub term_end_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ScheduleMeetingRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 10, max = 2000))]
    pub agenda: String,
    
    pub scheduled_for: DateTime<Utc>,
    
    pub duration_minutes: Option<i32>,
    
    pub location: Option<String>,
    
    pub meeting_type: Option<String>, // "regular", "emergency", "special"
    
    pub required_attendance: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CommitteeQuery {
    #[validate(range(min = 1))]
    pub page: Option<u32>,
    
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,
    
    pub scope: Option<String>,
    
    pub status: Option<String>,
    
    pub member_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CommitteeResponse {
    pub committee_id: i32,
    pub name: String,
    pub purpose: String,
    pub scope: String,
    pub formation_date: chrono::NaiveDate,
    pub status: String,
    pub member_count: i32,
    pub active_cases: i32,
    pub completed_cases: i32,
    pub chairperson: Option<MemberInfo>,
    pub rules: Option<serde_json::Value>,
    pub term_duration_months: Option<i32>,
    pub next_election_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct MemberInfo {
    pub member_id: i32,
    pub name: String,
    pub role: String,
    pub join_date: chrono::NaiveDate,
    pub term_end_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct CommitteeListResponse {
    pub committees: Vec<CommitteeResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct CommitteeMembersResponse {
    pub committee_id: i32,
    pub members: Vec<CommitteeMemberResponse>,
    pub total_members: i32,
    pub active_members: i32,
}

#[derive(Debug, Serialize)]
pub struct CommitteeMemberResponse {
    pub membership_id: i32,
    pub member_id: i32,
    pub name: String,
    pub role: String,
    pub join_date: chrono::NaiveDate,
    pub status: String,
    pub term_end_date: Option<chrono::NaiveDate>,
    pub contribution_score: Option<f64>,
    pub meetings_attended: i32,
    pub votes_cast: i32,
}

#[derive(Debug, Serialize)]
pub struct CommitteeMeetingResponse {
    pub meeting_id: i32,
    pub committee_id: i32,
    pub title: String,
    pub agenda: String,
    pub scheduled_for: DateTime<Utc>,
    pub duration_minutes: i32,
    pub location: Option<String>,
    pub meeting_type: String,
    pub status: String, // "scheduled", "in_progress", "completed", "cancelled"
    pub minutes: Option<String>,
    pub decisions: Vec<MeetingDecision>,
    pub attendees: Vec<MeetingAttendee>,
    pub recording_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MeetingDecision {
    pub decision_id: i32,
    pub title: String,
    pub description: String,
    pub vote_result: String,
    pub approved: bool,
    pub voting_details: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct MeetingAttendee {
    pub member_id: i32,
    pub name: String,
    pub role: String,
    pub attended: bool,
    pub join_time: Option<DateTime<Utc>>,
    pub leave_time: Option<DateTime<Utc>>,
}

pub async fn create_committee(
    request: web::Json<CreateCommitteeRequest>,
) -> impl Responder {
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let committee_request = request.into_inner();
    
    HttpResponse::Created().json(serde_json::json!({
        "message": "Committee created successfully",
        "committee": {
            "name": committee_request.name,
            "scope": committee_request.scope,
            "committee_id": 1
        }
    }))
}

pub async fn get_committees(
    query: web::Query<CommitteeQuery>,
) -> impl Responder {
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committees": [],
        "query": query_params,
        "total": 0
    }))
}

pub async fn get_committee(
    path: web::Path<i32>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "name": "Example Committee",
        "scope": "plagiarism"
    }))
}

pub async fn get_committee_members(
    path: web::Path<i32>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "members": [],
        "total_members": 0
    }))
}

pub async fn add_committee_member(
    path: web::Path<i32>,
    request: web::Json<AddCommitteeMemberRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let member_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "member_id": member_request.member_id,
        "role": member_request.role,
        "message": "Member added successfully"
    }))
}

pub async fn remove_committee_member(
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (committee_id, member_id) = path.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "member_id": member_id,
        "message": "Member removed successfully"
    }))
}

pub async fn get_committee_meetings(
    path: web::Path<i32>,
    query: web::Query<MeetingQuery>,
) -> impl Responder {
    let committee_id = path.into_inner();
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "meetings": [],
        "query": query_params
    }))
}

#[derive(Debug, Deserialize)]
pub struct MeetingQuery {
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
}

pub async fn schedule_meeting(
    path: web::Path<i32>,
    request: web::Json<ScheduleMeetingRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let meeting_request = request.into_inner();
    
    HttpResponse::Created().json(serde_json::json!({
        "committee_id": committee_id,
        "meeting_id": 1,
        "title": meeting_request.title,
        "scheduled_for": meeting_request.scheduled_for,
        "message": "Meeting scheduled successfully"
    }))
}

pub async fn get_committee_decisions(
    path: web::Path<i32>,
    query: web::Query<DecisionQuery>,
) -> impl Responder {
    let committee_id = path.into_inner();
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "decisions": [],
        "query": query_params
    }))
}

#[derive(Debug, Deserialize)]
pub struct DecisionQuery {
    pub decision_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub approved_only: Option<bool>,
    pub limit: Option<u32>,
}

pub async fn assign_case_to_committee(
    path: web::Path<i32>,
    request: web::Json<AssignCaseRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let assign_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "case_id": assign_request.case_id,
        "assignee_id": assign_request.assignee_id,
        "priority": assign_request.priority,
        "message": "Case assigned successfully"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct AssignCaseRequest {
    #[validate(range(min = 1))]
    pub case_id: i32,
    
    pub assignee_id: Option<i32>,
    
    pub priority: Option<String>,
    
    pub deadline: Option<DateTime<Utc>>,
}

pub async fn update_committee_status(
    path: web::Path<i32>,
    request: web::Json<UpdateStatusRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let status_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "new_status": status_request.status,
        "reason": status_request.reason,
        "message": "Committee status updated successfully"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStatusRequest {
    #[validate(length(min = 1, max = 20))]
    pub status: String,
    
    #[validate(length(min = 10, max = 500))]
    pub reason: String,
}

pub async fn get_committee_performance(
    path: web::Path<i32>,
    query: web::Query<PerformanceQuery>,
) -> impl Responder {
    let committee_id = path.into_inner();
    let query_params = query.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "performance_metrics": {
            "cases_resolved": 0,
            "avg_resolution_time": 0.0,
            "member_participation": 0.0,
            "decision_accuracy": 0.0
        },
        "period": query_params.period
    }))
}

#[derive(Debug, Deserialize)]
pub struct PerformanceQuery {
    pub period: Option<String>, // "week", "month", "quarter", "year"
}

pub async fn send_committee_invitation(
    path: web::Path<i32>,
    request: web::Json<CommitteeInvitationRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let invitation_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "invitee_id": invitation_request.invitee_id,
        "role": invitation_request.role,
        "invitation_id": "inv_123",
        "message": "Invitation sent successfully"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CommitteeInvitationRequest {
    #[validate(range(min = 1))]
    pub invitee_id: i32,
    
    #[validate(length(min = 1, max = 50))]
    pub role: String,
    
    pub message: Option<String>,
    
    pub expires_in_days: Option<i32>,
}

pub async fn record_committee_decision(
    path: web::Path<i32>,
    request: web::Json<RecordDecisionRequest>,
) -> impl Responder {
    let committee_id = path.into_inner();
    
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": validation_errors
        }));
    }
    
    let decision_request = request.into_inner();
    
    HttpResponse::Ok().json(serde_json::json!({
        "committee_id": committee_id,
        "decision_id": 1,
        "title": decision_request.title,
        "approved": decision_request.approved,
        "message": "Decision recorded successfully"
    }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct RecordDecisionRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 10, max = 2000))]
    pub description: String,
    
    pub approved: bool,
    
    pub voting_details: serde_json::Value,
    
    pub meeting_id: Option<i32>,
}

pub async fn export_committee_report(
    path: web::Path<i32>,
    query: web::Query<CommitteeReportQuery>,
) -> impl Responder {
    let committee_id = path.into_inner();
    let query_params = query.into_inner();
    
    // Generate report
    let report_data = format!("Committee Report #{}", committee_id);
    
    HttpResponse::Ok()
        .content_type(match query_params.format.as_str() {
            "pdf" => "application/pdf",
            "csv" => "text/csv",
            _ => "application/json",
        })
        .append_header(("Content-Disposition", format!("attachment; filename=\"committee_{}_report_{}.{}\"", 
            committee_id, 
            chrono::Utc::now().format("%Y%m%d"),
            query_params.format)))
        .body(report_data)
}

#[derive(Debug, Deserialize)]
pub struct CommitteeReportQuery {
    pub format: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub include_members: Option<bool>,
    pub include_decisions: Option<bool>,
    pub include_cases: Option<bool>,
}
