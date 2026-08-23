use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use crate::infrastructure::database::repositories::author_repository;

pub async fn get_authors(repo: web::Data<AuthorRepository>) -> impl Responder {
    match repo.get_all().await {
        Ok(authors) => {
            let count = authors.len();
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Found {} authors", count),
                "count": count,
                "data": authors
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))
    }
}

pub async fn get_author_by_orcid(
    repo: web::Data<AuthorRepository>,
    path: web::Path<String>
) -> impl Responder {
    let orcid = path.into_inner();
    
    match repo.get_by_orcid(&orcid).await {
        Ok(Some(author)) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": author
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "success": false,
            "error": format!("Author with ORCID {} not found", orcid)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))
    }
}
