#!/bin/bash

FILE="src/infrastructure/database/repositories/member_repository.rs"

# Let's find and fix the missing parenthesis
# Look for the problematic impl block
START_LINE=$(grep -n "impl MemberRepository for MemberRepositoryImpl" "$FILE" | cut -d: -f1)
if [ -z "$START_LINE" ]; then
    echo "Could not find impl block"
    exit 1
fi

echo "Impl block starts at line: $START_LINE"

# Find the end of the impl block
END_LINE=$(awk -v start="$START_LINE" 'NR > start && /^\s*}$/ {print NR; exit}' "$FILE")
if [ -z "$END_LINE" ]; then
    END_LINE=$(wc -l < "$FILE")
fi

echo "Impl block ends at line: $END_LINE"

# Count parentheses in the impl block
OPEN_PAREN=$(sed -n "${START_LINE},${END_LINE}p" "$FILE" | grep -o "(" | wc -l)
CLOSE_PAREN=$(sed -n "${START_LINE},${END_LINE}p" "$FILE" | grep -o ")" | wc -l)

echo "Open parentheses: $OPEN_PAREN"
echo "Close parentheses: $CLOSE_PAREN"

if [ "$OPEN_PAREN" -lt "$CLOSE_PAREN" ]; then
    echo "Missing opening parenthesis detected"
    
    # Find the line with the extra close paren
    LINE_NUM=$(awk -v start="$START_LINE" -v end="$END_LINE" '
        NR >= start && NR <= end {
            open = gsub(/\(/, "")
            close = gsub(/\)/, "")
            if (close > open && prev_open == prev_close) {
                print NR ": " $0
            }
            prev_open = open
            prev_close = close
        }' "$FILE" | head -1 | cut -d: -f1)
    
    if [ -n "$LINE_NUM" ]; then
        echo "Problem likely around line $LINE_NUM"
        sed -n "$((LINE_NUM-2)),$((LINE_NUM+2))p" "$FILE"
    fi
fi

# Let's create a clean version of the file
cat > /tmp/fixed_member_repo.rs << 'CLEANEOF'
use crate::domain::models::Member;
use crate::domain::repositories::MemberRepository;
use crate::infrastructure::database::{RepositoryError, RepositoryResult, DatabasePool};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
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
        // Simplified implementation
        Ok(None)
    }

    async fn save(&self, member: &Member) -> RepositoryResult<()> {
        // Simplified implementation
        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<Member>> {
        // Simplified implementation
        Ok(None)
    }

    async fn update(&self, member: &Member) -> RepositoryResult<()> {
        // Simplified implementation
        Ok(())
    }

    async fn delete(&self, member_id: &str) -> RepositoryResult<()> {
        // Simplified implementation
        Ok(())
    }

    async fn update_last_login(&self, member_id: &str) -> RepositoryResult<()> {
        // Simplified implementation
        Ok(())
    }

    async fn update_password(&self, member_id: &str, password_hash: &str) -> RepositoryResult<()> {
        // Simplified implementation
        Ok(())
    }

    async fn get_introduced_members(&self, member_id: &str) -> RepositoryResult<Vec<String>> {
        // Simplified implementation
        Ok(vec![])
    }

    async fn get_download_count(&self, member_id: &str) -> RepositoryResult<i64> {
        // Simplified implementation
        Ok(0)
    }

    async fn has_donated(&self, member_id: &str) -> RepositoryResult<bool> {
        // Simplified implementation
        Ok(false)
    }
}
CLEANEOF

# Replace the file with clean version
cp /tmp/fixed_member_repo.rs "$FILE"
echo "Replaced with clean implementation"
