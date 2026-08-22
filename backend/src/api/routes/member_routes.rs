use actix_web::web;
use crate::api::handlers::member_handler;

pub fn member_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            .route("", web::get().to(member_handler::get_members))
    );
}
