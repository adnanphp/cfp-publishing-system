use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::models::*;
use crate::infrastructure::database::RepositoryError;

#[async_trait]
pub trait TextRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Text>, RepositoryError>;
    async fn save(&self, text: &Text) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait MemberRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Member>, RepositoryError>;
    async fn save(&self, member: &Member) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait DonationRepository: Send + Sync {
    async fn save(&self, donation: &Donation) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait CharityRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Charity>, RepositoryError>;
}

#[async_trait]
pub trait PlagiarismRepository: Send + Sync {
    async fn save(&self, case: &PlagiarismCase) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait CommitteeRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Committee>, RepositoryError>;
}

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn save(&self, notification: &Notification) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait DownloadRepository: Send + Sync {
    async fn save(&self, download: &Download) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait VoteRepository: Send + Sync {
    async fn save(&self, vote: &Vote) -> Result<(), RepositoryError>;
}
