#!/bin/bash

echo "Restoring application/dto module..."
echo "==================================="

# Create directory
mkdir -p src/application/dto

# Check what's in backup
echo "Files in backup dto directory:"
ls -la ./src.backup.1769027748/application/dto/ 2>/dev/null || echo "No dto directory"

# Copy files
echo ""
echo "Copying DTO files..."
cp ./src.backup.1769027748/application/dto/*.rs src/application/dto/ 2>/dev/null || echo "No files to copy"

# Also copy subdirectories
if [ -d "./src.backup.1769027748/application/dto/requests" ]; then
    mkdir -p src/application/dto/requests
    cp ./src.backup.1769027748/application/dto/requests/*.rs src/application/dto/requests/ 2>/dev/null || true
fi

if [ -d "./src.backup.1769027748/application/dto/responses" ]; then
    mkdir -p src/application/dto/responses
    cp ./src.backup.1769027748/application/dto/responses/*.rs src/application/dto/responses/ 2>/dev/null || true
fi

if [ -d "./src.backup.1769027748/application/dto/types" ]; then
    mkdir -p src/application/dto/types
    cp ./src.backup.1769027748/application/dto/types/*.rs src/application/dto/types/ 2>/dev/null || true
fi

# Update application/mod.rs to include dto
if ! grep -q "pub mod dto" src/application/mod.rs; then
    echo "pub mod dto;" >> src/application/mod.rs
fi

echo ""
echo "Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ DTO module compiles successfully!"
    echo ""
    echo "Next: Test build and run"
    cargo build
    cargo run
else
    echo "❌ DTO module has compilation errors"
    echo ""
    echo "Creating minimal DTOs instead..."
    
    # Create minimal DTOs
    cat > src/application/dto/mod.rs << 'EOF'
pub mod requests;
pub mod responses;
pub mod types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTextRequest {
    pub title: String,
    pub content: String,
    pub author_id: String,
}

#[derive(Debug, Serialize)]
pub struct TextResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub status: String,
}
EOF
    
    # Create minimal submodules
    mkdir -p src/application/dto/{requests,responses,types}
    echo "// Request DTOs" > src/application/dto/requests/mod.rs
    echo "// Response DTOs" > src/application/dto/responses/mod.rs
    echo "// Type definitions" > src/application/dto/types/mod.rs
    
    echo "Testing minimal DTOs..."
    cargo check && echo "✅ Minimal DTOs work" || echo "❌ Still has errors"
fi
