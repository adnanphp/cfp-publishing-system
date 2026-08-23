pub mod auth_request;
pub mod member_request;
pub mod text_request;
pub mod donation_request;
pub mod plagiarism_request;
pub mod vote_request;

// Re-export for convenience
pub use auth_request::*;
pub use member_request::*;
pub use text_request::*;
pub use donation_request::*;
pub use plagiarism_request::*;
pub use vote_request::*;
