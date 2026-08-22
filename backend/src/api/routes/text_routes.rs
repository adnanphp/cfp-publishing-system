use actix_web::web;
use crate::api::handlers::text_handler;

pub fn text_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/texts")
            .route("", web::get().to(text_handler::get_texts))
            .route("/{id}", web::get().to(text_handler::get_text_by_id))
            .route("/author/{orcid}", web::get().to(text_handler::get_texts_by_author))
    );
}
