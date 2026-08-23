#!/bin/bash

echo "=== Adding Modules Back Step by Step ==="

echo "1. Creating proper lib.rs..."
cat > src/lib.rs << 'LIB'
pub mod api;
pub mod application;
pub mod config;
pub mod domain;
pub mod infrastructure;
pub mod utils;
LIB

echo "2. Creating module directories..."
mkdir -p src/{config,domain,application,infrastructure,api,utils}

echo "3. Creating mod.rs files..."
touch src/config/mod.rs
touch src/domain/mod.rs
touch src/application/mod.rs
touch src/infrastructure/mod.rs
touch src/api/mod.rs
touch src/utils/mod.rs

echo "4. Testing compilation with empty modules..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Empty modules work! Now let's add config module first..."
    
    # Copy your config files from backup
    echo "Looking for config files in backup..."
    if [ -d "../cfp-backup-*" ]; then
        BACKUP_DIR=$(ls -td ../cfp-backup-* | head -1)
        echo "Found backup: $BACKUP_DIR"
        
        # Copy config if exists
        if [ -d "$BACKUP_DIR/src/config" ]; then
            echo "Copying config module..."
            cp -r "$BACKUP_DIR/src/config/"* src/config/ 2>/dev/null || true
        fi
        
        # Create minimal config if needed
        if [ ! -f "src/config/mod.rs" ] || [ ! -s "src/config/mod.rs" ]; then
            cat > src/config/mod.rs << 'CONFIG'
pub mod app;
pub mod database;
pub mod redis;
pub mod jwt;

pub use app::*;
pub use database::*;
CONFIG
        fi
        
        # Create minimal app.rs if missing
        if [ ! -f "src/config/app.rs" ]; then
            cat > src/config/app.rs << 'APP'
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: u64,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // For now, return default config
        Ok(AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://localhost/cfp".to_string(),
            },
            redis: RedisConfig {
                url: "redis://localhost/".to_string(),
            },
            jwt: JwtConfig {
                secret: "default-secret-change-me".to_string(),
                expiration_hours: 24,
            },
        })
    }
}
APP
        fi
    fi
    
    echo "Testing compilation with config module..."
    cargo check
    
    if [ $? -eq 0 ]; then
        echo "✅ Config module added successfully!"
        echo ""
        echo "Next, update main.rs to use config:"
        echo ""
        echo "Current server works. To add more features:"
        echo "1. Add database connection"
        echo "2. Add domain models"
        echo "3. Add API routes"
        echo ""
        echo "Which module would you like to add next?"
    else
        echo "⚠️  Config module has issues. Let's comment it out and try another module."
        mv src/config/mod.rs src/config/mod.rs.backup
        echo "// Temporarily empty" > src/config/mod.rs
    fi
else
    echo "❌ Something went wrong with module structure"
fi
