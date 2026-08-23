#!/bin/bash

echo "Creating minimal working version..."

# Create minimal member repository
cat > src/infrastructure/database/repositories/member_repository.rs << 'MEMBERREPO'
use crate::domain::models::Member;
use crate::domain::repositories::MemberRepository;
use crate::infrastructure::database::RepositoryResult;
use async_trait::async_trait;
use sqlx::PgPool;

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
    async fn find_by_id(&self, _member_id: &str) -> RepositoryResult<Option<Member>> {
        Ok(None)
    }

    async fn save(&self, _member: &Member) -> RepositoryResult<()> {
        Ok(())
    }
}
MEMBERREPO

# Create other minimal repositories
for repo in text donation charity plagiarism committee notification download; do
    cat > src/infrastructure/database/repositories/${repo}_repository.rs << "REPOEOF"
use async_trait::async_trait;
use crate::infrastructure::database::RepositoryResult;

pub struct ${repo^}RepositoryImpl;

impl ${repo^}RepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::${repo^}Repository for ${repo^}RepositoryImpl {
    // Minimal implementations
}
REPOEOF
done

# Create minimal domain models
cat > src/domain/models/member.rs << 'MEMBERMODEL'
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub member_id: Uuid,
    pub name: String,
    pub organization: Option<String>,
    pub pseudonym: Option<String>,
    pub primary_email: String,
    pub recovery_email: Option<String>,
    pub password_hash: String,
    pub verification_matrix: Option<String>,
    pub matrix_expiry: Option<DateTime<Utc>>,
    pub join_date: DateTime<Utc>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub introduced_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
MEMBERMODEL

echo "Minimal version created"
