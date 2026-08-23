use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("Get profile")
}

pub async fn update_profile(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Update profile")
}

pub async fn get_messages() -> impl Responder {
    HttpResponse::Ok().body("Get messages")
}

pub async fn get_message(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Get message")
}

pub async fn send_message(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Send message")
}

pub async fn get_downloads() -> impl Responder {
    HttpResponse::Ok().body("Get downloads")
}

pub async fn get_member_donations() -> impl Responder {
    HttpResponse::Ok().body("Get member donations")
}

pub async fn get_member_comments() -> impl Responder {
    HttpResponse::Ok().body("Get member comments")
}
