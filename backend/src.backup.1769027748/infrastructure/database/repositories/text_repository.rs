use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

pub struct TextRepositoryImpl;

impl TextRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::TextRepository for TextRepositoryImpl {
    // Minimal implementations
}
