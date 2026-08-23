use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn get_donations() -> impl Responder {
    HttpResponse::Ok().body("Get donations")
}

pub async fn confirm_donation(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Confirm donation")
}

pub async fn refund_donation(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Refund donation")
}

pub async fn get_charities() -> impl Responder {
    HttpResponse::Ok().body("Get charities")
}

pub async fn get_charity(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Get charity")
}

pub async fn verify_payment(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Verify payment")
}
