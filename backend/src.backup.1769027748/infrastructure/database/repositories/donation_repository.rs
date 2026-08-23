use async_trait::async_trait;
use crate::infrastructure::database::database_pool::RepositoryResult;

pub struct DonationRepositoryImpl;

impl DonationRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::domain::repositories::DonationRepository for DonationRepositoryImpl {
    // Minimal implementations
}
