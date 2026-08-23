use sqlx::PgPool;
use crate::domain::models::author::Author;

pub async fn get_author_by_orcid(pool: &PgPool, orcid: &str) -> Result<Option<Author>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT orcid, member_id, bio, specialization, h_index, total_downloads
        FROM authors 
        WHERE orcid = $1
        "#,
        orcid
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Author {
        orcid: r.orcid,
        member_id: r.member_id as u32,
        bio: r.bio,
        specialization: r.specialization,
        h_index: r.h_index.unwrap_or(0),
        total_downloads: r.total_downloads.unwrap_or(0),
    }))
}

pub async fn list_authors(pool: &PgPool) -> Result<Vec<Author>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT orcid, member_id, bio, specialization, h_index, total_downloads
        FROM authors 
        ORDER BY orcid
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Author {
            orcid: r.orcid,
            member_id: r.member_id as u32,
            bio: r.bio,
            specialization: r.specialization,
            h_index: r.h_index.unwrap_or(0),
            total_downloads: r.total_downloads.unwrap_or(0),
        })
        .collect())
}
