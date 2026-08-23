use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::responses::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMemberRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_member(_req: web::Json<CreateMemberRequest>) -> impl Responder {
    let response = MemberResponse {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        name: "Test User".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Created().json(ApiResponse::new(response, "Member created".to_string()))
}

pub async fn get_member(_path: web::Path<Uuid>) -> impl Responder {
    let response = MemberResponse {
        id: Uuid::new_v4(),
        email: "user@example.com".to_string(),
        name: "Existing User".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(response, "Member found".to_string()))
}

pub async fn update_member(_path: web::Path<Uuid>, _req: web::Json<UpdateMemberRequest>) -> impl Responder {
    let response = MemberResponse {
        id: Uuid::new_v4(),
        email: "updated@example.com".to_string(),
        name: "Updated User".to_string(),
        created_at: chrono::Utc::now(),
    };
    HttpResponse::Ok().json(ApiResponse::new(response, "Member updated".to_string()))
}

pub async fn delete_member(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({"deleted": true}),
        "Member deleted".to_string()
    ))
}

pub async fn list_members() -> impl Responder {
    let members = vec![
        MemberResponse {
            id: Uuid::new_v4(),
            email: "user1@example.com".to_string(),
            name: "User One".to_string(),
            created_at: chrono::Utc::now(),
        },
        MemberResponse {
            id: Uuid::new_v4(),
            email: "user2@example.com".to_string(),
            name: "User Two".to_string(),
            created_at: chrono::Utc::now(),
        },
    ];
    HttpResponse::Ok().json(ApiResponse::new(members, "Members list".to_string()))
}

pub async fn get_member_stats(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::new(
        serde_json::json!({
            "texts_count": 5,
            "donations_count": 3,
            "downloads_count": 12
        }),
        "Member stats".to_string()
    ))
}
