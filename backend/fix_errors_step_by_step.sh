#!/bin/bash

echo "=== Step 1: Add Missing Dependencies ==="
if ! grep -q "rand = " Cargo.toml; then
    echo 'rand = "0.8"' >> Cargo.toml
fi
if ! grep -q "actix = " Cargo.toml; then
    echo 'actix = "0.13"' >> Cargo.toml
fi
if ! grep -q "actix-web-actors" Cargo.toml; then
    echo 'actix-web-actors = "4.0"' >> Cargo.toml
fi

echo "=== Step 2: Fix api::responses imports ==="
echo "Fixing handler imports..."
find src/api/handlers -name "*.rs" -exec sed -i 's|crate::api::responses|crate::application::dto::responses|g' {} \;
sed -i 's|crate::api::responses|crate::application::dto::responses|g' src/api/handlers/mod.rs
sed -i 's|api::responses|application::dto::responses|g' src/application/queries/plagiarism_queries.rs

echo "=== Step 3: Comment out Axum imports (you're using Actix, not Axum) ==="
find src -name "*.rs" -exec sed -i 's/^use axum;/\/\/ use axum;/g' {} \;
find src -name "*.rs" -exec sed -i 's/^use axum::/\/\/ use axum::/g' {} \;

echo "=== Step 4: Fix missing response types ==="
# Create missing response types or comment out
sed -i '/use crate::application::dto::types::StatusCount/s/^/# /g' src/application/dto/responses/text_response.rs

# Comment out problematic plagiarism response imports temporarily
sed -i '4,8s/^/# /g' src/application/queries/plagiarism_queries.rs

echo "=== Step 5: Update dependencies ==="
cargo update

echo "=== Step 6: Test Compilation ==="
cargo check 2>&1 | head -50
