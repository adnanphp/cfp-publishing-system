use actix_web::{web, HttpResponse, get};
use sqlx::PgPool;

#[get("/api/downloads/test")]
pub async fn test_downloads(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!("SELECT COUNT(*) as count FROM downloads")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let count = row.count.unwrap_or(0);
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Downloads table test",
                "download_count": count,
                "test_passed": true
            }))
        },
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", err),
            "test_passed": false
        })),
    }
}

#[get("/api/downloads/sample")]
pub async fn sample_downloads(pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!("SELECT download_id, member_id, text_id, download_date, country FROM downloads LIMIT 5")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let downloads: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                serde_json::json!({
                    "download_id": row.download_id,
                    "member_id": row.member_id,
                    "text_id": row.text_id,
                    "download_date": row.download_date.to_string(),
                    "country": row.country
                })
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "count": downloads.len(),
                "downloads": downloads
            }))
        },
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", err),
        })),
    }
}
