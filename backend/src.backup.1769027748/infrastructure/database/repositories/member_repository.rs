use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::Member;

#[async_trait]
pub trait MemberRepository: Send + Sync {
    async fn find_by_id(&self, member_id: Uuid) -> Result<Option<Member>, String>;
    async fn save(&self, member: Member) -> Result<(), String>;
}

pub struct MemberRepositoryImpl;

impl MemberRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MemberRepository for MemberRepositoryImpl {
    async fn find_by_id(&self, _member_id: Uuid) -> Result<Option<Member>, String> {
        Ok(None)
    }
    
    async fn save(&self, _member: Member) -> Result<(), String> {
        Ok(())
    }
}
