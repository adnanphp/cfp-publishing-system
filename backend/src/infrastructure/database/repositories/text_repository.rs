use sqlx::PgPool;
use crate::domain::models::text::Text;

pub async fn get_text_by_id(pool: &PgPool, text_id: i32) -> Result<Option<Text>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            text_id, author_orcid, title, abstract as abstract_text, topic, 
            version, upload_date, status, download_count
        FROM texts 
        WHERE text_id = $1
        "#,
        text_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Text {
        text_id: r.text_id as u32,
        author_orcid: r.author_orcid,
        title: r.title,
        abstract_text: r.abstract_text,
        topic: r.topic,
        version: r.version as u32,
        upload_date: r.upload_date.unwrap_or_default(),
        status: r.status.unwrap_or_else(|| "draft".to_string()),
        download_count: r.download_count.unwrap_or(0) as u32,
    }))
}

pub async fn list_texts_by_author(pool: &PgPool, author_orcid: &str) -> Result<Vec<Text>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            text_id, author_orcid, title, abstract as abstract_text, topic, 
            version, upload_date, status, download_count
        FROM texts 
        WHERE author_orcid = $1
        ORDER BY upload_date DESC
        "#,
        author_orcid
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Text {
            text_id: r.text_id as u32,
            author_orcid: r.author_orcid,
            title: r.title,
            abstract_text: r.abstract_text,
            topic: r.topic,
            version: r.version as u32,
            upload_date: r.upload_date.unwrap_or_default(),
            status: r.status.unwrap_or_else(|| "draft".to_string()),
            download_count: r.download_count.unwrap_or(0) as u32,
        })
        .collect())
}
