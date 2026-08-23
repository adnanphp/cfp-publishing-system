// backend/src/domain/mod.rs
pub mod models;
pub mod enums;
pub mod value_objects;
pub mod aggregates;
pub mod events;

// Re-exports
pub use models::*;
pub use enums::*;
pub use value_objects::*;
pub mod repositories;
