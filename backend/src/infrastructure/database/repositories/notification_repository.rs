use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

pub struct NotificationRepositoryImpl;

impl NotificationRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::NotificationRepository for NotificationRepositoryImpl {
    // Minimal implementations
}
