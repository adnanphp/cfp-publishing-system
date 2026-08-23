#!/bin/bash

echo "Checking backup for missing domain objects..."
echo "=============================================="

# Check for value objects in backup
echo "Looking for value objects..."
find ./src.backup.1769027748/domain/ -name "*.rs" -type f | grep -i -E "(address|phone|email)" || echo "No value object files found"

# Check domain/value_objects directory
echo ""
echo "Contents of domain/value_objects in backup:"
ls -la ./src.backup.1769027748/domain/value_objects/ 2>/dev/null || echo "No value_objects directory"

# Check domain/enums directory
echo ""
echo "Contents of domain/enums in backup:"
ls -la ./src.backup.1769027748/domain/enums/ 2>/dev/null || echo "No enums directory"

# Check Cargo.toml for axum
echo ""
echo "Checking Cargo.toml for axum..."
grep -i axum Cargo.toml || echo "axum not in Cargo.toml"

# Check original DTOs that use axum
echo ""
echo "Checking backup api_response.rs for axum usage:"
head -20 ./src.backup.1769027748/application/dto/responses/api_response.rs 2>/dev/null || echo "api_response.rs not found"
