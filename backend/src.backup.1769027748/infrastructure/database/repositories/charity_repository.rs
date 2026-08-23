use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

pub struct CharityRepositoryImpl;

impl CharityRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::CharityRepository for CharityRepositoryImpl {
    // Minimal implementations
}
