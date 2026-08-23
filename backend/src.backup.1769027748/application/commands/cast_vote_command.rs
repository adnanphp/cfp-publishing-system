use async_trait::async_trait;
use uuid::Uuid;

use crate::utils::error::AppError;

// Simple stub implementation
pub struct Command;
pub struct CommandHandler;

#[async_trait]
impl crate::application::commands::CommandHandler<Command, Uuid> for CommandHandler {
    async fn handle(&self, _command: Command) -> Result<Uuid, AppError> {
        Ok(Uuid::new_v4())
    }
}
