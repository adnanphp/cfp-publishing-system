use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn download_file(_path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().body("Download handler")
}
