#!/bin/bash

BACKUP="$1"

if [ -z "$BACKUP" ]; then
    echo "Usage: $0 /path/to/backup"
    exit 1
fi

# Files to restore in order (safe ones first)
SAFE_FILES=(
    "domain/enums/mod.rs"
    "domain/models/mod.rs"
    "application/dto/requests/mod.rs"
    "application/dto/responses/mod.rs"
    "utils/error.rs"
    "utils/validation.rs"
)

echo "Starting gradual restoration from: $BACKUP"
echo "=========================================="

for file in "${SAFE_FILES[@]}"; do
    echo -e "\n--- Processing: $file ---"
    
    if [ -f "$BACKUP/$file" ]; then
        # Create backup of current file if it exists
        if [ -f "src/$file" ]; then
            cp "src/$file" "src/$file.bak"
        fi
        
        # Create directory
        mkdir -p "src/$(dirname "$file")"
        
        # Copy file
        cp "$BACKUP/$file" "src/$file"
        echo "Copied: $file"
        
        # Test compilation
        echo "Testing compilation..."
        if cargo check --quiet 2>/dev/null; then
            echo "✅ Compiles successfully!"
        else
            echo "❌ Compilation failed. Restoring backup..."
            
            # Show first few errors
            cargo check 2>&1 | grep -A 2 "error\[E" | head -10
            
            # Restore backup if we had one
            if [ -f "src/$file.bak" ]; then
                cp "src/$file.bak" "src/$file"
                echo "Restored from backup"
            else
                # Create minimal placeholder
                echo "// Placeholder - original caused errors" > "src/$file"
                echo "Created placeholder"
            fi
            
            # Save problematic file for later
            mkdir -p "problematic_files/$(dirname "$file")"
            cp "$BACKUP/$file" "problematic_files/$file"
            echo "Problematic file saved to: problematic_files/$file"
            
            read -p "Press enter to continue..." -n1
        fi
    else
        echo "⚠️  File not found in backup"
    fi
done

echo -e "\n--- Next, let's check utils/mod.rs ---"
if [ -f "$BACKUP/utils/mod.rs" ]; then
    echo "Backing up current utils/mod.rs..."
    cp "src/utils/mod.rs" "src/utils/mod.rs.bak"
    
    echo "Creating safe version without serde_yaml..."
    # Create a filtered version without serde_yaml issues
    grep -v "serde_yaml" "$BACKUP/utils/mod.rs" > /tmp/utils_temp.rs
    
    # Check if it has serde_yaml imports
    if grep -q "serde_yaml" "$BACKUP/utils/mod.rs"; then
        echo "Found serde_yaml references. Creating safe version..."
        # Remove serde_yaml imports and related functions
        grep -v "use.*serde_yaml" /tmp/utils_temp.rs | \
            grep -v "serde_yaml::" | \
            grep -v "fn.*to_yaml" > /tmp/utils_safe.rs
        cp /tmp/utils_safe.rs "src/utils/mod.rs"
    else
        cp /tmp/utils_temp.rs "src/utils/mod.rs"
    fi
    
    echo "Testing compilation..."
    if cargo check --quiet 2>/dev/null; then
        echo "✅ utils/mod.rs compiles successfully!"
    else
        echo "❌ Still has issues. Keeping minimal version."
        cp "src/utils/mod.rs.bak" "src/utils/mod.rs"
    fi
fi

echo -e "\n--- Summary ---"
echo "Safe files restored. Next, you can restore more complex files."
echo "Problematic files saved in: problematic_files/"
echo ""
echo "To continue, you can restore:"
echo "1. domain/value_objects/"
echo "2. domain/aggregates/"
echo "3. domain/repositories/"
echo "4. api/handlers/ (one at a time)"
echo ""
echo "Run: find '$BACKUP' -name '*.rs' | grep -i 'handler'"
