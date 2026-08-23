use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

pub struct PlagiarismRepositoryImpl;

impl PlagiarismRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::PlagiarismRepository for PlagiarismRepositoryImpl {
    // Minimal implementations
}
