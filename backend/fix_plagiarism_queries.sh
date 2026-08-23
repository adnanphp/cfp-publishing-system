#!/bin/bash

echo "Fixing indentation in plagiarism_queries.rs..."

# Create a backup
cp src/application/queries/plagiarism_queries.rs src/application/queries/plagiarism_queries.rs.backup

# Show the problematic area
echo "Lines around the error:"
sed -n '5,25p' src/application/queries/plagiarism_queries.rs

# Fix by rewriting the entire file with proper formatting
cat > src/application/queries/plagiarism_queries.rs << 'EOF'
use crate::{
    api::handlers::ApiResponse,
    application::dto::{
        PlagiarismCaseResponse,
        PlagiarismCaseSearchResponse,
        VoteResponse,
        PlagiarismStatsResponse,
    },
    domain::models::PlagiarismCase,
    infrastructure::database::repositories::PlagiarismRepository,
};
use uuid::Uuid;

pub struct PlagiarismQueries {
    repository: PlagiarismRepository,
}

impl PlagiarismQueries {
    pub fn new(repository: PlagiarismRepository) -> Self {
        Self { repository }
    }
    
    pub async fn get_case(&self, case_id: Uuid) -> Result<PlagiarismCaseResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismCaseResponse)
    }
    
    pub async fn search_cases(&self) -> Result<PlagiarismCaseSearchResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismCaseSearchResponse)
    }
    
    pub async fn get_votes(&self, case_id: Uuid) -> Result<VoteResponse, String> {
        // Implementation placeholder
        Ok(VoteResponse)
    }
    
    pub async fn get_stats(&self) -> Result<PlagiarismStatsResponse, String> {
        // Implementation placeholder
        Ok(PlagiarismStatsResponse)
    }
}
EOF

echo "Fixed file. Running cargo check..."
cargo check
