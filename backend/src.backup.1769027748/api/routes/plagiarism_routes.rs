use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_cases() -> impl Responder {
    HttpResponse::Ok().body("Get cases")
}

pub async fn get_case(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Get case")
}

pub async fn create_case(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Create case")
}

pub async fn vote_on_case(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Vote on case")
}

pub async fn appeal_case(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Appeal case")
}

pub async fn close_case(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Close case")
}

pub async fn bulk_check_plagiarism(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Bulk check plagiarism")
}
