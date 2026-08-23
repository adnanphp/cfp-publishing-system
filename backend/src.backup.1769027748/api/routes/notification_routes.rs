use actix_web::{web, HttpResponse, Responder};

pub async fn broadcast_notification(_body: web::Json<()>) -> impl Responder {
    HttpResponse::Ok().body("Broadcast notification")
}

pub async fn test_notification() -> impl Responder {
    HttpResponse::Ok().body("Test notification")
}
