#!/bin/bash

echo "Fixing async_trait usage..."

# For command files
for file in src/application/commands/*.rs; do
    if [ -f "$file" ]; then
        # Remove any existing async_trait line
        sed -i '/^#\[macro_use\]/d' "$file"
        # Add proper async_trait import
        sed -i '1i use async_trait::async_trait;' "$file"
        # Fix the attribute
        sed -i 's/^#\[async_trait\]$/#[async_trait]/' "$file"
    fi
done

# For query files
for file in src/application/queries/*.rs; do
    if [ -f "$file" ]; then
        # Remove any existing async_trait line
        sed -i '/^#\[macro_use\]/d' "$file"
        # Add proper async_trait import
        sed -i '1i use async_trait::async_trait;' "$file"
        # Fix the attribute
        sed -i 's/^#\[async_trait\]$/#[async_trait]/' "$file"
    fi
done

echo "Async trait usage fixed"
