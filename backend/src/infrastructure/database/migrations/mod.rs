// This module is for SQLx migration files
// The actual migration files (.sql) should be in the migrations/ directory
// This file is just a placeholder to make the module structure work

pub mod migration_manager;

// Re-export for convenience
pub use migration_manager::*;
