#!/bin/bash

echo "Cleaning up duplicate module files..."
echo "===================================="

# Remove duplicate single .rs files (keep the mod.rs directories)
for dir in api application infrastructure; do
    if [ -d "src/$dir" ]; then
        echo "Checking: $dir"
        
        # For each submodule that has both .rs and /mod.rs
        for module in middleware routes services queries commands database cache security messaging external responses handlers; do
            single_file="src/$dir/$module.rs"
            mod_dir="src/$dir/$module"
            
            if [ -f "$single_file" ] && [ -d "$mod_dir" ]; then
                echo "  Removing duplicate: $single_file (keeping $mod_dir/)"
                rm -f "$single_file"
            fi
        done
    fi
done

# Also check for duplicate mod.rs declarations
echo ""
echo "Cleaning up mod.rs files..."
for mod_file in src/api/mod.rs src/application/mod.rs src/infrastructure/mod.rs; do
    if [ -f "$mod_file" ]; then
        echo "Cleaning: $mod_file"
        # Remove duplicate lines
        sort -u "$mod_file" > "${mod_file}.tmp"
        mv "${mod_file}.tmp" "$mod_file"
    fi
done

echo ""
echo "Testing compilation..."
cargo check
