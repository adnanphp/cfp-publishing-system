#!/bin/bash

echo "Cleaning previous builds..."
cargo clean

echo "Checking syntax..."
if cargo check --lib; then
    echo "Syntax check passed!"
    echo "Building project..."
    if cargo build; then
        echo "Build successful! 🎉"
        echo ""
        echo "You can now run:"
        echo "1. cargo run"
        echo "2. Or test with: cargo test"
    else
        echo "Build failed. Checking errors..."
        cargo build 2>&1 | grep -A5 -B5 "error:"
    fi
else
    echo "Syntax check failed:"
    cargo check --lib 2>&1 | grep -A5 -B5 "error:"
fi
