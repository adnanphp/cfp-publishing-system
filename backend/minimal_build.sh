#!/bin/bash

echo "Creating minimal build..."

# Clean first
cargo clean

# Comment out problematic middleware in mod.rs
sed -i 's/^pub mod \([a-z_]*\);$/\/\/ pub mod \1;  \/\/ Temporarily disabled/' src/api/middleware/mod.rs

# Keep only error_middleware for now
echo "pub mod error_middleware;" >> src/api/middleware/mod.rs

# Comment out problematic routes
sed -i 's/^.*committee_handler::.*$/\/\/ &/' src/api/routes/mod.rs

# Build
echo "Attempting build..."
cargo build 2>&1 | grep -A5 -B5 "error:"

# If build fails, try cargo check
if [ $? -ne 0 ]; then
    echo "Build failed, trying cargo check..."
    cargo check --lib
fi
