pub mod member_events;
pub mod donation_events;
pub mod plagiarism_events;
pub mod text_events;

// Re-export for convenience
pub use member_events::*;
pub use donation_events::*;
pub use plagiarism_events::*;
pub use text_events::*;

/// Trait for all domain events
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> uuid::Uuid;
    fn timestamp(&self) -> chrono::DateTime<chrono::Utc>;
    fn version(&self) -> i32 {
        1
    }
}
