use actix_web::web;
use crate::api::handlers::auth_handler;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_handler::login))
            .route("/register", web::post().to(auth_handler::register))
            .route("/logout", web::post().to(auth_handler::logout))
    );
}
