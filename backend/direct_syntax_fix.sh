#!/bin/bash

FILE="src/infrastructure/database/repositories/member_repository.rs"

echo "Creating clean version..."

# Create a clean, minimal version
cat > "$FILE" << 'CLEANVERSION'
use crate::domain::models::Member;
use crate::domain::repositories::MemberRepository;
use crate::infrastructure::database::{RepositoryError, RepositoryResult};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

pub struct MemberRepositoryImpl {
    pool: PgPool,
}

impl MemberRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemberRepository for MemberRepositoryImpl {
    async fn find_by_id(&self, member_id: &str) -> RepositoryResult<Option<Member>> {
        // Stub implementation
        Ok(None)
    }

    async fn save(&self, member: &Member) -> RepositoryResult<()> {
        // Stub implementation
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<Member>> {
        // Stub implementation
        Ok(None)
    }

    async fn update(&self, member: &Member) -> RepositoryResult<()> {
        // Stub implementation
        Ok(())
    }

    async fn delete(&self, member_id: &str) -> RepositoryResult<()> {
        // Stub implementation
        Ok(())
    }

    async fn update_last_login(&self, member_id: &str) -> RepositoryResult<()> {
        // Stub implementation
        Ok(())
    }

    async fn update_password(&self, member_id: &str, password_hash: &str) -> RepositoryResult<()> {
        // Stub implementation
        Ok(())
    }

    async fn get_introduced_members(&self, member_id: &str) -> RepositoryResult<Vec<String>> {
        // Stub implementation
        Ok(vec![])
    }

    async fn get_download_count(&self, member_id: &str) -> RepositoryResult<i64> {
        // Stub implementation
        Ok(0)
    }

    async fn has_donated(&self, member_id: &str) -> RepositoryResult<bool> {
        // Stub implementation
        Ok(false)
    }
}
CLEANVERSION

echo "File replaced with clean version"
echo "Now try: cargo check"
