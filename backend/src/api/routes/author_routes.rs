use actix_web::web;
use crate::api::handlers::author_handler;

pub fn author_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authors")
            .route("", web::get().to(author_handler::get_authors))
            .route("/{orcid}", web::get().to(author_handler::get_author_by_orcid))
    );
}
