pub mod database;
pub mod redis;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub database: database::DatabaseConfig,
    pub redis: redis::RedisConfig,
    pub jwt_secret: String,
    pub environment: String,
}
