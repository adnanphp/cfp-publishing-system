//! Integration tests for the CFP system

mod auth_tests;
mod donation_tests;
mod plagiarism_tests;

pub use auth_tests::*;
pub use donation_tests::*;
pub use plagiarism_tests::*;

/// Integration test setup
pub mod setup {
    use super::super::utils;
    use crate::config::Config;
    use crate::app::AppState;
    
    /// Create test application state
    pub async fn create_test_app_state() -> AppState {
        utils::init_test_env();
        
        let config = Config::from_env().expect("Failed to load config");
        let db_pool = utils::create_test_db().await;
        
        AppState::new(config, db_pool).await.expect("Failed to create app state")
    }
    
    /// Clean up after tests
    pub async fn cleanup() {
        // Drop test database
        // In-memory SQLite is automatically cleaned up
    }
}
