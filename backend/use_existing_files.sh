#!/bin/bash

echo "=== Using Your Existing Files ==="

echo "1. Checking what config files you have..."
ls -la src/config/

echo "2. Based on what you have, let's fix the mod.rs..."
if [ -f "src/config/database.rs" ]; then
    echo "Found database.rs"
    DB_EXISTS=true
else
    echo "No database.rs found"
    DB_EXISTS=false
fi

if [ -f "src/config/redis.rs" ]; then
    echo "Found redis.rs"
    REDIS_EXISTS=true
else
    echo "No redis.rs found"
    REDIS_EXISTS=false
fi

if [ -f "src/config/jwt.rs" ]; then
    echo "Found jwt.rs"
    JWT_EXISTS=true
else
    echo "No jwt.rs found"
    JWT_EXISTS=false
fi

echo "3. Creating mod.rs based on what you have..."
cat > src/config/mod.rs << 'MODRS'
pub mod app;

// Only include modules that exist
$([ "$DB_EXISTS" = "true" ] && echo "pub mod database;")
$([ "$REDIS_EXISTS" = "true" ] && echo "pub mod redis;")
$([ "$JWT_EXISTS" = "true" ] && echo "pub mod jwt;")

pub use app::*;

$([ "$DB_EXISTS" = "true" ] && echo "pub use database::*;")
$([ "$REDIS_EXISTS" = "true" ] && echo "pub use redis::*;")
$([ "$JWT_EXISTS" = "true" ] && echo "pub use jwt::*;")
MODRS

# Actually create the file properly
cat > src/config/mod.rs << 'MODRSACTUAL'
pub mod app;

// Only include modules that exist
MODRSACTUAL

# Append based on what exists
if [ "$DB_EXISTS" = "true" ]; then
    echo "pub mod database;" >> src/config/mod.rs
fi
if [ "$REDIS_EXISTS" = "true" ]; then
    echo "pub mod redis;" >> src/config/mod.rs
fi
if [ "$JWT_EXISTS" = "true" ]; then
    echo "pub mod jwt;" >> src/config/mod.rs
fi

echo "" >> src/config/mod.rs
echo "pub use app::*;" >> src/config/mod.rs
echo "" >> src/config/mod.rs

if [ "$DB_EXISTS" = "true" ]; then
    echo "pub use database::*;" >> src/config/mod.rs
fi
if [ "$REDIS_EXISTS" = "true" ]; then
    echo "pub use redis::*;" >> src/config/mod.rs
fi
if [ "$JWT_EXISTS" = "true" ]; then
    echo "pub use jwt::*;" >> src/config/mod.rs
fi

echo "4. Creating missing files if needed..."
if [ "$DB_EXISTS" = "false" ]; then
    echo "Creating database.rs placeholder..."
    cat > src/config/database.rs << 'DB'
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}
DB
fi

if [ "$REDIS_EXISTS" = "false" ]; then
    echo "Creating redis.rs placeholder..."
    cat > src/config/redis.rs << 'REDIS'
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

impl RedisConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}
REDIS
fi

if [ "$JWT_EXISTS" = "false" ]; then
    echo "Creating jwt.rs placeholder..."
    cat > src/config/jwt.rs << 'JWT'
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: u64,
}

impl JwtConfig {
    pub fn new(secret: &str, expiration_hours: u64) -> Self {
        Self {
            secret: secret.to_string(),
            expiration_hours,
        }
    }
}
JWT
fi

echo "5. Checking if app.rs exists..."
if [ ! -f "src/config/app.rs" ]; then
    echo "Creating app.rs..."
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

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // For now, return default config
        Ok(AppConfig {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: DatabaseConfig::new("postgres://localhost/cfp"),
            redis: RedisConfig::new("redis://localhost/"),
            jwt: JwtConfig::new("default-secret-change-me", 24),
        })
    }
}
APP
else
    echo "app.rs already exists"
fi

echo "6. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Using your existing file structure."
    echo ""
    echo "Now run: cargo run"
    echo "Test: curl http://localhost:3000/api/config"
else
    echo "❌ Still errors. Let's see what's in your files..."
    find src/config -name "*.rs" -exec echo "=== {} ===" \; -exec head -20 {} \;
    
    echo ""
    echo "Let's simplify..."
    cat > src/config/mod.rs << 'SIMPLE'
pub mod app;
pub use app::*;
SIMPLE
    
    cargo check
fi
