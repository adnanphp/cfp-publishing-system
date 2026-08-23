use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde_json::json;

pub async fn get_downloads(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query(
        r#"
        SELECT 
            download_id,
            member_id,
            text_id,
            download_date,
            ip_address,
            user_agent,
            country
        FROM downloads
        ORDER BY download_date DESC
        LIMIT 50
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            let downloads: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                json!({
                    "download_id": row.download_id,
                    "member_id": row.member_id,
                    "text_id": row.text_id,
                    "download_date": row.download_date.to_string(),
                    "ip_address": row.ip_address,
                    "user_agent": row.user_agent,
                    "country": row.country
                })
            }).collect();
            
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Found {} downloads", downloads.len()),
                "count": downloads.len(),
                "data": downloads
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": []
            }))
        }
    }
}

pub async fn get_download_count(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT COUNT(*) as count FROM downloads")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let count = row.count.unwrap_or(0);
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": "Total download count",
                "data": {
                    "total_downloads": count
                }
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": {}
            }))
        }
    }
}

pub async fn get_download_stats(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query(
        r#"
        SELECT 
            COUNT(*) as total_downloads,
            COUNT(DISTINCT member_id) as unique_members,
            COUNT(DISTINCT text_id) as unique_texts,
            COALESCE(COUNT(*) FILTER (WHERE download_date > CURRENT_DATE - INTERVAL '7 days'), 0) as last_7_days
        FROM downloads
        "#
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(stats) => {
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": "Download statistics",
                "data": {
                    "total_downloads": stats.total_downloads.unwrap_or(0),
                    "unique_members": stats.unique_members.unwrap_or(0),
                    "unique_texts": stats.unique_texts.unwrap_or(0),
                    "last_7_days": stats.last_7_days
                }
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": {}
            }))
        }
    }
}

pub async fn get_downloads_by_text(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let text_id = path.into_inner();
    
    match sqlx::query(
        r#"
        SELECT 
            COUNT(*) as download_count,
            COUNT(DISTINCT member_id) as unique_downloaders
        FROM downloads 
        WHERE text_id = $1
        "#,
        text_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            let count = row.download_count.unwrap_or(0);
            let unique = row.unique_downloaders.unwrap_or(0);
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Download stats for text ID {}", text_id),
                "data": {
                    "text_id": text_id,
                    "download_count": count,
                    "unique_downloaders": unique
                }
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": {}
            }))
        }
    }
}

pub async fn get_top_downloaded_texts(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query(
        r#"
        SELECT 
            t.text_id,
            t.title,
            COUNT(d.download_id) as download_count
        FROM texts t
        LEFT JOIN downloads d ON t.text_id = d.text_id
        GROUP BY t.text_id, t.title
        ORDER BY download_count DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => {
            let texts: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                json!({
                    "text_id": row.text_id,
                    "title": row.title,
                    "download_count": row.download_count.unwrap_or(0)
                })
            }).collect();
            
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": "Top downloaded texts",
                "data": texts
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": []
            }))
        }
    }
}

pub async fn get_downloads_by_member(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let member_id = path.into_inner();
    
    match sqlx::query(
        r#"
        SELECT 
            COUNT(*) as download_count,
            COUNT(DISTINCT text_id) as unique_texts
        FROM downloads 
        WHERE member_id = $1
        "#,
        member_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            let count = row.download_count.unwrap_or(0);
            let unique = row.unique_texts.unwrap_or(0);
            HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("Download stats for member ID {}", member_id),
                "data": {
                    "member_id": member_id,
                    "total_downloads": count,
                    "unique_texts_downloaded": unique
                }
            }))
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Database error: {}", err),
                "data": {}
            }))
        }
    }
}
