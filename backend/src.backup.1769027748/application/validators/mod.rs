pub mod donation_validator;
pub mod member_validator;
pub mod text_validator;
pub mod vote_validator;

// Re-export for convenience
pub use donation_validator::*;
pub use member_validator::*;
pub use text_validator::*;
pub use vote_validator::*;

use crate::utils::error::AppError;

pub trait Validator<T> {
    fn validate(&self, data: &T) -> Result<(), AppError>;
}
