use sqlx::{Error, PgPool};
use crate::domain::models::Download;
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Clone)]
pub struct DownloadRepository {
    pool: PgPool,
}

impl DownloadRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn get_all(&self) -> Result<Vec<Download>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                download_id, member_id, text_id, download_date, 
                ip_address, user_agent, country
            FROM downloads 
            ORDER BY download_date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let downloads: Vec<Download> = rows.into_iter().map(|row| {
            // Convert NaiveDateTime to DateTime<Utc>
            let download_date = DateTime::<Utc>::from_naive_utc_and_offset(row.download_date, Utc);
            
            Download {
                download_id: row.download_id as u32,
                member_id: row.member_id as u32,
                text_id: row.text_id as u32,
                download_date,
                ip_address: row.ip_address,
                user_agent: row.user_agent,
                country: row.country,
            }
        }).collect();
        
        Ok(downloads)
    }
    
    pub async fn get_by_id(&self, download_id: u32) -> Result<Option<Download>, Error> {
        let row = sqlx::query(
            r#"
            SELECT 
                download_id, member_id, text_id, download_date, 
                ip_address, user_agent, country
            FROM downloads 
            WHERE download_id = $1
            "#,
            download_id as i32
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| {
            let download_date = DateTime::<Utc>::from_naive_utc_and_offset(r.download_date, Utc);
            
            Download {
                download_id: r.download_id as u32,
                member_id: r.member_id as u32,
                text_id: r.text_id as u32,
                download_date,
                ip_address: r.ip_address,
                user_agent: r.user_agent,
                country: r.country,
            }
        }))
    }
    
    pub async fn get_by_member(&self, member_id: u32) -> Result<Vec<Download>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                download_id, member_id, text_id, download_date, 
                ip_address, user_agent, country
            FROM downloads 
            WHERE member_id = $1
            ORDER BY download_date DESC
            "#,
            member_id as i32
        )
        .fetch_all(&self.pool)
        .await?;
        
        let downloads: Vec<Download> = rows.into_iter().map(|row| {
            let download_date = DateTime::<Utc>::from_naive_utc_and_offset(row.download_date, Utc);
            
            Download {
                download_id: row.download_id as u32,
                member_id: row.member_id as u32,
                text_id: row.text_id as u32,
                download_date,
                ip_address: row.ip_address,
                user_agent: row.user_agent,
                country: row.country,
            }
        }).collect();
        
        Ok(downloads)
    }
    
    pub async fn get_by_text(&self, text_id: u32) -> Result<Vec<Download>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                download_id, member_id, text_id, download_date, 
                ip_address, user_agent, country
            FROM downloads 
            WHERE text_id = $1
            ORDER BY download_date DESC
            "#,
            text_id as i32
        )
        .fetch_all(&self.pool)
        .await?;
        
        let downloads: Vec<Download> = rows.into_iter().map(|row| {
            let download_date = DateTime::<Utc>::from_naive_utc_and_offset(row.download_date, Utc);
            
            Download {
                download_id: row.download_id as u32,
                member_id: row.member_id as u32,
                text_id: row.text_id as u32,
                download_date,
                ip_address: row.ip_address,
                user_agent: row.user_agent,
                country: row.country,
            }
        }).collect();
        
        Ok(downloads)
    }
    
    pub async fn get_total_downloads(&self) -> Result<u32, Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM downloads")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.count.unwrap_or(0) as u32)
    }
    
    pub async fn get_downloads_by_date_range(
        &self, 
        start_date: DateTime<Utc>, 
        end_date: DateTime<Utc>
    ) -> Result<Vec<Download>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                download_id, member_id, text_id, download_date, 
                ip_address, user_agent, country
            FROM downloads 
            WHERE download_date BETWEEN $1 AND $2
            ORDER BY download_date DESC
            "#,
            start_date.naive_utc(),
            end_date.naive_utc()
        )
        .fetch_all(&self.pool)
        .await?;
        
        let downloads: Vec<Download> = rows.into_iter().map(|row| {
            let download_date = DateTime::<Utc>::from_naive_utc_and_offset(row.download_date, Utc);
            
            Download {
                download_id: row.download_id as u32,
                member_id: row.member_id as u32,
                text_id: row.text_id as u32,
                download_date,
                ip_address: row.ip_address,
                user_agent: row.user_agent,
                country: row.country,
            }
        }).collect();
        
        Ok(downloads)
    }
    
    pub async fn create(&self, download: &Download) -> Result<Download, Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO downloads (member_id, text_id, download_date, ip_address, user_agent, country)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING download_id, member_id, text_id, download_date, ip_address, user_agent, country
            "#,
            download.member_id as i32,
            download.text_id as i32,
            download.download_date.naive_utc(),
            download.ip_address.as_deref(),
            download.user_agent.as_deref(),
            download.country.as_deref()
        )
        .fetch_one(&self.pool)
        .await?;
        
        let download_date = DateTime::<Utc>::from_naive_utc_and_offset(row.download_date, Utc);
        
        Ok(Download {
            download_id: row.download_id as u32,
            member_id: row.member_id as u32,
            text_id: row.text_id as u32,
            download_date,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            country: row.country,
        })
    }
    
    pub async fn get_downloads_by_country(&self) -> Result<Vec<(String, u32)>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT country, COUNT(*) as count
            FROM downloads
            WHERE country IS NOT NULL
            GROUP BY country
            ORDER BY count DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let result: Vec<(String, u32)> = rows.into_iter()
            .map(|row| (row.country.unwrap_or("Unknown".to_string()), row.count.unwrap_or(0) as u32))
            .collect();
        
        Ok(result)
    }
    
    pub async fn get_popular_texts(&self, limit: u32) -> Result<Vec<(u32, u32)>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT text_id, COUNT(*) as download_count
            FROM downloads
            GROUP BY text_id
            ORDER BY download_count DESC
            LIMIT $1
            "#,
            limit as i32
        )
        .fetch_all(&self.pool)
        .await?;
        
        let result: Vec<(u32, u32)> = rows.into_iter()
            .map(|row| (row.text_id as u32, row.download_count.unwrap_or(0) as u32))
            .collect();
        
        Ok(result)
    }
}
