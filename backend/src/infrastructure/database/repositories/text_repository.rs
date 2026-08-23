use sqlx::{Error, PgPool};
use crate::domain::models::Text;

<<<<<<< HEAD
#[derive(Clone)]
pub struct TextRepository {
    pool: PgPool,
}

impl TextRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self) -> Result<Vec<Text>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                text_id, author_orcid, title, abstract as abstract_text, topic, 
                version, upload_date::text, status, download_count, 
                total_donations, avg_rating
            FROM texts 
            ORDER BY text_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let texts: Vec<Text> = rows.into_iter().map(|row| {
            // Convert BigDecimal to f64
            let total_donations = row.total_donations
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            let avg_rating = row.avg_rating
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            Text {
                text_id: row.text_id as u32,
                author_orcid: row.author_orcid,
                title: row.title,
                abstract_text: row.abstract_text,
                topic: row.topic,
                version: row.version as u32,
                upload_date: row.upload_date.unwrap_or_default(),
                status: row.status.unwrap_or_else(|| "draft".to_string()),
                download_count: row.download_count.unwrap_or(0) as u32,
                total_donations,
                avg_rating,
            }
        }).collect();
        
        Ok(texts)
    }
    
    pub async fn get_by_id(&self, text_id: u32) -> Result<Option<Text>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT 
                text_id, author_orcid, title, abstract as abstract_text, topic, 
                version, upload_date::text, status, download_count, 
                total_donations, avg_rating
            FROM texts 
            WHERE text_id = $1
            "#,
            text_id as i32
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| {
            // Convert BigDecimal to f64
            let total_donations = r.total_donations
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            let avg_rating = r.avg_rating
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            Text {
                text_id: r.text_id as u32,
                author_orcid: r.author_orcid,
                title: r.title,
                abstract_text: r.abstract_text,
                topic: r.topic,
                version: r.version as u32,
                upload_date: r.upload_date.unwrap_or_default(),
                status: r.status.unwrap_or_else(|| "draft".to_string()),
                download_count: r.download_count.unwrap_or(0) as u32,
                total_donations,
                avg_rating,
            }
        }))
    }
    
    pub async fn get_by_author(&self, author_orcid: &str) -> Result<Vec<Text>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                text_id, author_orcid, title, abstract as abstract_text, topic, 
                version, upload_date::text, status, download_count, 
                total_donations, avg_rating
            FROM texts 
            WHERE author_orcid = $1
            ORDER BY text_id
            "#,
            author_orcid
        )
        .fetch_all(&self.pool)
        .await?;
        
        let texts: Vec<Text> = rows.into_iter().map(|row| {
            // Convert BigDecimal to f64
            let total_donations = row.total_donations
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            let avg_rating = row.avg_rating
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            
            Text {
                text_id: row.text_id as u32,
                author_orcid: row.author_orcid,
                title: row.title,
                abstract_text: row.abstract_text,
                topic: row.topic,
                version: row.version as u32,
                upload_date: row.upload_date.unwrap_or_default(),
                status: row.status.unwrap_or_else(|| "draft".to_string()),
                download_count: row.download_count.unwrap_or(0) as u32,
                total_donations,
                avg_rating,
            }
        }).collect();
        
        Ok(texts)
    }
=======
pub async fn get_text_by_id(pool: &PgPool, text_id: i32) -> Result<Option<Text>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            text_id, 
            author_orcid, 
            title, 
            abstract as abstract_text, 
            topic, 
            version, 
            upload_date, 
            status, 
            download_count,
            0 as total_donations,
            0.0 as avg_rating
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
        total_donations: r.total_donations,
        avg_rating: r.avg_rating,
    }))
}

pub async fn list_texts_by_author(pool: &PgPool, author_orcid: &str) -> Result<Vec<Text>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            text_id, 
            author_orcid, 
            title, 
            abstract as abstract_text, 
            topic, 
            version, 
            upload_date, 
            status, 
            download_count,
            0 as total_donations,
            0.0 as avg_rating
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
            total_donations: r.total_donations,
            avg_rating: r.avg_rating,
        })
        .collect())
>>>>>>> d6b82885d9fdeabeaa372f1b131dc4b183715ff8
}
