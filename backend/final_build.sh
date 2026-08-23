#!/bin/bash

echo "Cleaning previous builds..."
cargo clean

echo "Updating dependencies..."
cargo update

echo "Checking compilation..."
cargo check --lib

if [ $? -eq 0 ]; then
    echo "Compilation successful! Building..."
    cargo build
else
    echo "Compilation failed. Showing errors:"
    cargo check --lib 2>&1 | grep -A5 -B5 "error:"
fi
