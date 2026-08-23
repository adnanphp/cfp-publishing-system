// backend/src/api/mod.rs
pub mod routes;
pub mod middleware;
pub mod handlers;

// Re-exports
pub use routes::configure_routes;
