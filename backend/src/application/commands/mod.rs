pub mod create_member_command;
pub mod create_donation_command;
pub mod cast_vote_command;
pub mod update_text_command;

// Re-export for convenience
pub use create_member_command::*;
pub use create_donation_command::*;
pub use cast_vote_command::*;
pub use update_text_command::*;

use crate::utils::error::AppError;

#[async_trait]
pub trait CommandHandler<TCommand, TResult> {
    async fn handle(&self, command: TCommand) -> Result<TResult, AppError>;
}

pub trait Command: Send + Sync {
    fn command_name(&self) -> &'static str;
}
use async_trait::async_trait;
