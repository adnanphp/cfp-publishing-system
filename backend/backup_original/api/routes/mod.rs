use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(api_test))
            .route("/status", web::get().to(status))
    );
}

async fn api_test() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body("CFP Backend API - Test Endpoint")
}

async fn status() -> actix_web::HttpResponse {
    use crate::domain::enums::{MemberStatus, TextStatus};
    
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "operational",
        "version": "1.0.0",
        "enums": {
            "member_status": format!("{:?}", [
                MemberStatus::Pending,
                MemberStatus::Active,
                MemberStatus::Suspended,
            ]),
            "text_status": format!("{:?}", [
                TextStatus::Draft,
                TextStatus::Published,
                TextStatus::Archived,
            ])
        }
    }))
}
