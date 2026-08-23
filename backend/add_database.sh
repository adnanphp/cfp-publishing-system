#!/bin/bash

echo "=== Adding Database Module ==="

echo "1. Adding SQLx dependency..."
if ! grep -q "sqlx" Cargo.toml; then
    cat >> Cargo.toml << 'DEPS'

# Database
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "chrono", "uuid", "migrate"] }
tokio = { version = "1.0", features = ["full"] }
