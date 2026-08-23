use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::{BasicCache, CacheError, CacheResult, CacheKey, Ttl};
use crate::domain::models::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub member_id: Uuid,
    pub email: String,
    pub name: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub last_accessed_at: u64,
    pub expires_at: u64,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub is_refresh_token: bool,
    pub metadata: HashMap<String, String>,
}

impl Session {
    pub fn new(
        member: &Member,
        user_agent: Option<String>,
        ip_address: Option<String>,
        is_refresh_token: bool,
        session_duration: Duration,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        
        let expires_at = now + session_duration.as_secs();
        
        Self {
            session_id: Uuid::new_v4().to_string(),
            member_id: member.member_id,
            email: member.primary_email.to_string(),
            name: member.name.clone(),
            roles: Vec::new(), // Will be populated from member roles
            permissions: Vec::new(), // Will be populated from member permissions
            created_at: now,
            last_accessed_at: now,
            expires_at,
            user_agent,
            ip_address,
            is_refresh_token,
            metadata: HashMap::new(),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        
        now > self.expires_at
    }
    
    pub fn update_last_accessed(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        
        self.last_accessed_at = now;
    }
    
    pub fn extend(&mut self, duration: Duration) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
        
        self.expires_at = now + duration.as_secs();
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

pub struct SessionStore {
    cache_manager: Box<dyn BasicCache>,
}

impl SessionStore {
    pub fn new(cache_manager: Box<dyn BasicCache>) -> Self {
        Self { cache_manager }
    }
    
    pub async fn create_session(
        &self,
        member: &Member,
        user_agent: Option<String>,
        ip_address: Option<String>,
        is_refresh_token: bool,
    ) -> CacheResult<Session> {
        let session_duration = if is_refresh_token {
            Ttl::REFRESH_TOKEN
        } else {
            Ttl::SESSION
        };
        
        let mut session = Session::new(
            member,
            user_agent,
            ip_address,
            is_refresh_token,
            session_duration,
        );
        
        // Add member roles and permissions to session
        // This would be populated from member service
        
        // Save session to cache
        let session_key = CacheKey::session(&session.session_id);
        let user_sessions_key = CacheKey::user_sessions(&member.member_id.to_string());
        
        // Store session
        self.cache_manager
            .set(&session_key, &session, Some(session_duration))
            .await?;
        
        // Add session to user's session list
        self.cache_manager
            .set_add(&user_sessions_key, &session.session_id)
            .await?;
        
        // Set expiry on user sessions key
        self.cache_manager
            .expire(&user_sessions_key, session_duration)
            .await?;
        
        info!("Created new session for member: {}", member.member_id);
        
        Ok(session)
    }
    
    pub async fn get_session(&self, session_id: &str) -> CacheResult<Option<Session>> {
        let session_key = CacheKey::session(session_id);
        
        match self.cache_manager.get(&session_key).await {
            Ok(Some(mut session)) => {
                // Check if session is expired
                if session.is_expired() {
                    self.delete_session(session_id).await?;
                    return Ok(None);
                }
                
                // Update last accessed time
                session.update_last_accessed();
                
                // Update session in cache with new last accessed time
                let ttl = Duration::from_secs(session.expires_at - session.created_at);
                self.cache_manager
                    .set(&session_key, &session, Some(ttl))
                    .await?;
                
                Ok(Some(session))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    pub async fn delete_session(&self, session_id: &str) -> CacheResult<()> {
        let session_key = CacheKey::session(session_id);
        
        // Get session to find member_id for cleanup
        if let Ok(Some(session)) = self.cache_manager.get(&session_key).await {
            let user_sessions_key = CacheKey::user_sessions(&session.member_id.to_string());
            
            // Remove from user's session list
            self.cache_manager
                .set_remove(&user_sessions_key, session_id)
                .await?;
        }
        
        // Delete the session
        self.cache_manager.delete(&session_key).await?;
        
        info!("Deleted session: {}", session_id);
        
        Ok(())
    }
    
    pub async fn delete_all_user_sessions(&self, member_id: Uuid) -> CacheResult<u64> {
        let user_sessions_key = CacheKey::user_sessions(&member_id.to_string());
        
        // Get all session IDs for this user
        let session_ids = self.cache_manager
            .set_members(&user_sessions_key)
            .await?;
        
        let mut deleted_count = 0;
        
        // Delete each session
        for session_id in session_ids {
            if self.delete_session(&session_id).await.is_ok() {
                deleted_count += 1;
            }
        }
        
        // Delete the user sessions set
        self.cache_manager.delete(&user_sessions_key).await?;
        
        info!("Deleted all {} sessions for member: {}", deleted_count, member_id);
        
        Ok(deleted_count)
    }
    
    pub async fn refresh_session(&self, session_id: &str, duration: Duration) -> CacheResult<Option<Session>> {
        let session_key = CacheKey::session(session_id);
        
        match self.cache_manager.get(&session_key).await {
            Ok(Some(mut session)) => {
                if session.is_expired() {
                    self.delete_session(session_id).await?;
                    return Ok(None);
                }
                
                // Extend session
                session.extend(duration);
                session.update_last_accessed();
                
                // Update in cache
                self.cache_manager
                    .set(&session_key, &session, Some(duration))
                    .await?;
                
                // Also update user sessions key expiry
                let user_sessions_key = CacheKey::user_sessions(&session.member_id.to_string());
                self.cache_manager
                    .expire(&user_sessions_key, duration)
                    .await?;
                
                Ok(Some(session))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
    
    pub async fn validate_session(&self, session_id: &str) -> CacheResult<bool> {
        let session_key = CacheKey::session(session_id);
        
        match self.cache_manager.get::<Session>(&session_key).await {
            Ok(Some(session)) => {
                if session.is_expired() {
                    self.delete_session(session_id).await?;
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    pub async fn get_user_sessions(&self, member_id: Uuid) -> CacheResult<Vec<Session>> {
        let user_sessions_key = CacheKey::user_sessions(&member_id.to_string());
        
        // Get all session IDs for this user
        let session_ids = self.cache_manager
            .set_members(&user_sessions_key)
            .await?;
        
        let mut sessions = Vec::new();
        
        // Get each session
        for session_id in session_ids {
            if let Ok(Some(session)) = self.get_session(&session_id).await {
                sessions.push(session);
            }
        }
        
        Ok(sessions)
    }
    
    pub async fn cleanup_expired_sessions(&self) -> CacheResult<u64> {
        // This is a simplified implementation
        // In production, you might want to use Redis SCAN or a scheduled task
        
        warn!("Session cleanup is not implemented in this simplified version");
        Ok(0)
    }
    
    pub async fn get_session_stats(&self, member_id: Option<Uuid>) -> CacheResult<SessionStats> {
        let stats = if let Some(member_id) = member_id {
            let sessions = self.get_user_sessions(member_id).await?;
            let active_sessions = sessions.iter().filter(|s| !s.is_expired()).count();
            let expired_sessions = sessions.len() - active_sessions;
            
            SessionStats {
                total_sessions: sessions.len() as u64,
                active_sessions: active_sessions as u64,
                expired_sessions: expired_sessions as u64,
                member_id: Some(member_id),
            }
        } else {
            // System-wide stats would require scanning all sessions
            // This is simplified for now
            SessionStats {
                total_sessions: 0,
                active_sessions: 0,
                expired_sessions: 0,
                member_id: None,
            }
        };
        
        Ok(stats)
    }
    
    pub async fn add_session_metadata(
        &self,
        session_id: &str,
        key: String,
        value: String,
    ) -> CacheResult<()> {
        let session_key = CacheKey::session(session_id);
        
        match self.cache_manager.get(&session_key).await {
            Ok(Some(mut session)) => {
                session.add_metadata(key, value);
                
                // Update in cache
                let ttl = Duration::from_secs(session.expires_at - session.created_at);
                self.cache_manager
                    .set(&session_key, &session, Some(ttl))
                    .await?;
                
                Ok(())
            }
            Ok(None) => Err(CacheError::CacheMiss(session_key)),
            Err(e) => Err(e),
        }
    }
    
    pub async fn invalidate_sessions_by_pattern(&self, pattern: &str) -> CacheResult<u64> {
        // Note: Redis KEYS command is blocking and not recommended for production
        // This is a simplified implementation
        warn!("Pattern-based session invalidation is not implemented in this simplified version");
        Ok(0)
    }
}

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub expired_sessions: u64,
    pub member_id: Option<Uuid>,
}
