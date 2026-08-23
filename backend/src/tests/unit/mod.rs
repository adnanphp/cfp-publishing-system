//! Unit tests for the CFP system

mod domain_tests;
mod service_tests;
mod validator_tests;

pub use domain_tests::*;
pub use service_tests::*;
pub use validator_tests::*;

/// Unit test utilities
pub mod utils {
    use mockall::automock;
    
    /// Mock trait for testing
    #[automock]
    pub trait MockRepository {
        fn get_item(&self, id: i32) -> Option<String>;
        fn save_item(&mut self, item: String) -> Result<(), String>;
    }
    
    /// Test fixture builder
    pub struct TestFixtureBuilder<T> {
        data: T,
    }
    
    impl<T> TestFixtureBuilder<T> {
        pub fn new(initial: T) -> Self {
            Self { data: initial }
        }
        
        pub fn build(self) -> T {
            self.data
        }
    }
    
    /// Assert with custom message
    #[macro_export]
    macro_rules! assert_with_msg {
        ($condition:expr, $($arg:tt)*) => {
            if !$condition {
                panic!($($arg)*);
            }
        };
    }
}
