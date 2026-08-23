use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

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
