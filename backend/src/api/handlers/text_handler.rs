use actix_web::{HttpResponse, Responder, web};
use serde_json::json;
use crate::infrastructure::database::repositories::text_repository;

pub async fn get_texts(repo: web::Data<TextRepository>) -> impl Responder {
    match repo.get_all().await {
        Ok(texts) => {
            let count: usize = texts.len();
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Found {} texts", count),
                "count": count,
                "data": texts
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))
    }
}

pub async fn get_text_by_id(
    repo: web::Data<TextRepository>,
    path: web::Path<u32>
) -> impl Responder {
    let text_id = path.into_inner();
    
    match repo.get_by_id(text_id).await {
        Ok(Some(_text)) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Text found (placeholder)"
        })),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "success": false,
            "error": format!("Text with id {} not found", text_id)
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))
    }
}

pub async fn get_texts_by_author(
    repo: web::Data<TextRepository>,
    path: web::Path<String>
) -> impl Responder {
    let author_orcid = path.into_inner();
    
    match repo.get_by_author(&author_orcid).await {
        Ok(texts) => {
            let count: usize = texts.len();
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Found {} texts by author {}", count, author_orcid),
                "count": count,
                "data": texts
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "error": e.to_string()
        }))
    }
}
