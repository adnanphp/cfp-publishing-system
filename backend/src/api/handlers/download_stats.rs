use actix_web::{web, HttpResponse, get};
use sqlx::PgPool;

#[get("/api/downloads/stats")]
pub async fn download_stats(pool: web::Data<PgPool>) -> HttpResponse {
    // Get total downloads count
    let total_result = sqlx::query("SELECT COUNT(*) as count FROM downloads")
        .fetch_one(pool.get_ref())
        .await;
    
    // Get downloads by country
    let by_country_result = sqlx::query(
        "SELECT country, COUNT(*) as count FROM downloads 
         WHERE country IS NOT NULL 
         GROUP BY country ORDER BY count DESC LIMIT 5"
    )
    .fetch_all(pool.get_ref())
    .await;
    
    // Get popular texts
    let popular_texts_result = sqlx::query(
        "SELECT text_id, COUNT(*) as download_count FROM downloads 
         GROUP BY text_id ORDER BY download_count DESC LIMIT 5"
    )
    .fetch_all(pool.get_ref())
    .await;
    
    match (total_result, by_country_result, popular_texts_result) {
        (Ok(total_row), Ok(country_rows), Ok(text_rows)) => {
            let total = total_row.count.unwrap_or(0);
            
            let countries: Vec<serde_json::Value> = country_rows.into_iter().map(|row| {
                serde_json::json!({
                    "country": row.country.unwrap_or("Unknown".to_string()),
                    "count": row.count.unwrap_or(0)
                })
            }).collect();
            
            let popular_texts: Vec<serde_json::Value> = text_rows.into_iter().map(|row| {
                serde_json::json!({
                    "text_id": row.text_id,
                    "downloads": row.download_count.unwrap_or(0)
                })
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "total_downloads": total,
                "top_countries": countries,
                "popular_texts": popular_texts,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        },
        _ => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Failed to fetch download statistics"
        })),
    }
}
