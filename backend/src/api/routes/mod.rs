use actix_web::web;
use crate::api::handlers;

pub mod auth_routes;
pub mod member_routes;
pub mod author_routes;
pub mod text_routes;
pub mod download_routes;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Root-level routes (outside /api scope)
        .route("/", web::get().to(handlers::root))
        .route("/health", web::get().to(handlers::health_check))
        .route("/status", web::get().to(handlers::api_status))
        // API routes under /api scope
        .service(
            web::scope("/api")
                .route("/status", web::get().to(handlers::api_status))
                .route("/health", web::get().to(handlers::health_check))
                .route("/db-test", web::get().to(handlers::db_test))
                .configure(auth_routes::auth_routes)
                .configure(member_routes::member_routes)
                .configure(author_routes::author_routes)
                .configure(text_routes::text_routes)
               // .configure(download_routes::download_routes)
        );
}
