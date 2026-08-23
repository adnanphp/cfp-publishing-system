//! Test modules for the CFP system

pub mod integration;
pub mod unit;

/// Test utilities and helpers
pub mod utils {
    use std::sync::Once;
    use dotenv::dotenv;
    
    static INIT: Once = Once::new();
    
    /// Initialize test environment
    pub fn init_test_env() {
        INIT.call_once(|| {
            dotenv().ok();
            // Set test environment variables
            std::env::set_var("APP_ENVIRONMENT", "test");
            std::env::set_var("DATABASE_URL", "sqlite::memory:");
            std::env::set_var("JWT_SECRET", "test_secret_key_for_testing_only");
            std::env::set_var("TEST_MODE", "true");
        });
    }
    
    /// Create a test database connection
    #[cfg(feature = "database")]
    pub async fn create_test_db() -> sqlx::SqlitePool {
        use sqlx::sqlite::SqlitePoolOptions;
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create test database pool");
            
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
            
        pool
    }
    
    /// Mock data generators
    pub mod mock {
        use chrono::{Utc, Duration};
        use uuid::Uuid;
        
        /// Generate a test email
        pub fn test_email() -> String {
            format!("test-{}@example.com", Uuid::new_v4().simple())
        }
        
        /// Generate a test password
        pub fn test_password() -> String {
            "TestPassword123!".to_string()
        }
        
        /// Generate test ORCID
        pub fn test_orcid() -> String {
            format!("{:04}-{:04}-{:04}-{:03}X", 
                rand::random::<u16>() % 10000,
                rand::random::<u16>() % 10000,
                rand::random::<u16>() % 10000,
                rand::random::<u16>() % 1000
            )
        }
        
        /// Generate test name
        pub fn test_name() -> String {
            let names = vec!["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank"];
            let surnames = vec!["Smith", "Johnson", "Brown", "Davis", "Wilson", "Taylor"];
            
            format!("{} {}", 
                names[rand::random::<usize>() % names.len()],
                surnames[rand::random::<usize>() % surnames.len()]
            )
        }
        
        /// Generate test organization
        pub fn test_organization() -> String {
            let orgs = vec![
                "University of Technology",
                "Research Institute",
                "Open Source Foundation",
                "Academic Press",
                "Digital Library"
            ];
            orgs[rand::random::<usize>() % orgs.len()].to_string()
        }
        
        /// Generate test text title
        pub fn test_text_title() -> String {
            let adjectives = vec!["Advanced", "Modern", "Practical", "Theoretical", "Comprehensive"];
            let topics = vec!["Machine Learning", "Cryptography", "Database Systems", "Web Development", "System Design"];
            
            format!("{} Guide to {}", 
                adjectives[rand::random::<usize>() % adjectives.len()],
                topics[rand::random::<usize>() % topics.len()]
            )
        }
        
        /// Generate test donation amount
        pub fn test_donation_amount() -> f64 {
            let amounts = vec![10.0, 25.0, 50.0, 100.0, 250.0];
            amounts[rand::random::<usize>() % amounts.len()]
        }
        
        /// Generate future date
        pub fn future_date(days: i64) -> chrono::NaiveDate {
            (Utc::now() + Duration::days(days)).date_naive()
        }
        
        /// Generate past date
        pub fn past_date(days: i64) -> chrono::NaiveDate {
            (Utc::now() - Duration::days(days)).date_naive()
        }
    }
}
