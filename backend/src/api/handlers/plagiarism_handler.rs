use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckPlagiarismRequest {
    pub text: String,
    pub text_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlagiarismCaseResponse {
    pub id: Uuid,
    pub similarity_score: f64,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    pub vote: String, // "plagiarism" or "not_plagiarism"
    pub comment: Option<String>,
}

pub async fn check_plagiarism(_req: web::Json<CheckPlagiarismRequest>) -> impl Responder {
    let result = PlagiarismCaseResponse {
        id: Uuid::new_v4(),
        similarity_score: 0.75,
        status: "pending_review".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(result, "Plagiarism check completed".to_string()))
}

pub async fn get_plagiarism_case(_path: web::Path<Uuid>) -> impl Responder {
    let case = PlagiarismCaseResponse {
        id: Uuid::new_v4(),
        similarity_score: 0.82,
        status: "under_review".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(case, "Plagiarism case found".to_string()))
}

pub async fn list_plagiarism_cases() -> impl Responder {
    let cases = vec![
        PlagiarismCaseResponse {
            id: Uuid::new_v4(),
            similarity_score: 0.65,
            status: "resolved".to_string(),
            created_at: chrono::Utc::now(),
        },
        PlagiarismCaseResponse {
            id: Uuid::new_v4(),
            similarity_score: 0.90,
            status: "confirmed".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(cases, "Plagiarism cases list".to_string()))
}

pub async fn cast_vote(_path: web::Path<Uuid>, _req: web::Json<VoteRequest>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"vote_id": "vote_123", "status": "recorded"}),
        "Vote recorded".to_string()
    ))
}

pub async fn get_case_votes(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        vec![
            serde_json::json!({"voter": "user1", "vote": "plagiarism"}),
            serde_json::json!({"voter": "user2", "vote": "not_plagiarism"})
        ],
        "Case votes".to_string()
    ))
}

pub async fn resolve_plagiarism_case(_path: web::Path<Uuid>) -> impl Responder {
    let case = PlagiarismCaseResponse {
        id: Uuid::new_v4(),
        similarity_score: 0.88,
        status: "resolved".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(case, "Case resolved".to_string()))
}

pub async fn get_plagiarism_stats() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "total_cases": 150,
            "confirmed_cases": 75,
            "false_positives": 25,
            "pending_review": 50
        }),
        "Plagiarism stats".to_string()
    ))
}

pub async fn search_plagiarism_cases() -> impl Responder {
    let cases = vec![
        PlagiarismCaseResponse {
            id: Uuid::new_v4(),
            similarity_score: 0.70,
            status: "confirmed".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(cases, "Search results".to_string()))
}

pub async fn export_plagiarism_report(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        "Report content would be here",
        "Report exported".to_string()
    ))
}
