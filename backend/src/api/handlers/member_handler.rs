use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use crate::infrastructure::database::repositories::MemberRepository;

pub async fn get_members(_repo: web::Data<MemberRepository>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Members endpoint - Not implemented yet",
        "data": []
    }))
}
