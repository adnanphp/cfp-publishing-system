#!/bin/bash

echo "Running quick compilation check..."

# First, check syntax
if rustc --edition=2021 --crate-type=lib --emit=metadata src/lib.rs 2>/tmp/rustc_errors.txt; then
    echo "Syntax check passed!"
else
    echo "Syntax errors found:"
    head -20 /tmp/rustc_errors.txt
fi

# Try to compile just the library
echo "Attempting to compile library..."
cargo rustc --lib -- -Z no-codegen 2>&1 | grep -i error | head -10
