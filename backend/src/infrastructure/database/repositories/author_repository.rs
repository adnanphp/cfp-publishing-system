use sqlx::{Error, PgPool};
use crate::domain::models::Author;

#[derive(Clone)]
pub struct AuthorRepository {
    pool: PgPool,
}

impl AuthorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self) -> Result<Vec<Author>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT orcid, member_id, bio, specialization, h_index, total_downloads
            FROM authors 
            ORDER BY orcid
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let authors: Vec<Author> = rows.into_iter().map(|row| Author {
            orcid: row.orcid,
            member_id: row.member_id.map(|id| id as u32),
            bio: row.bio,
            specialization: row.specialization,
            h_index: row.h_index.unwrap_or(0),
            total_downloads: row.total_downloads.unwrap_or(0),
        }).collect();
        
        Ok(authors)
    }
    
    pub async fn get_by_orcid(&self, orcid: &str) -> Result<Option<Author>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT orcid, member_id, bio, specialization, h_index, total_downloads
            FROM authors 
            WHERE orcid = $1
            "#,
            orcid
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| Author {
            orcid: r.orcid,
            member_id: r.member_id.map(|id| id as u32),
            bio: r.bio,
            specialization: r.specialization,
            h_index: r.h_index.unwrap_or(0),
            total_downloads: r.total_downloads.unwrap_or(0),
        }))
    }
}
