#!/bin/bash

BACKUP="./src.backup.1769027748"

echo "Gradual restoration from backup: $BACKUP"
echo "========================================="

# Create directories for problematic files
mkdir -p problematic_files

# Function to test compilation
test_compilation() {
    echo -n "Testing compilation... "
    if cargo check --quiet 2>/dev/null; then
        echo "✅ OK"
        return 0
    else
        echo "❌ FAILED"
        # Show first error
        cargo check 2>&1 | grep -A 1 "error\[E" | head -5
        return 1
    fi
}

# Step 1: Restore safe utils files first (except mod.rs)
echo -e "\n=== Step 1: Restoring utils files ==="
UTILS_FILES=(
    "utils/error.rs"
    "utils/validation.rs"
    "utils/logger.rs"
    "utils/cryptography.rs"
    "utils/datetime.rs"
)

for file in "${UTILS_FILES[@]}"; do
    echo -e "\n--- $file ---"
    if [ -f "$BACKUP/$file" ]; then
        # Backup current file
        if [ -f "src/$file" ]; then
            cp "src/$file" "src/$file.bak"
        fi
        
        # Copy file
        mkdir -p "src/$(dirname "$file")"
        cp "$BACKUP/$file" "src/$file"
        echo "Copied"
        
        if ! test_compilation; then
            echo "Restoring backup..."
            if [ -f "src/$file.bak" ]; then
                cp "src/$file.bak" "src/$file"
            else
                echo "// Placeholder" > "src/$file"
            fi
            cp "$BACKUP/$file" "problematic_files/$file"
        fi
    else
        echo "Not found in backup"
    fi
done

# Step 2: Handle utils/mod.rs specially
echo -e "\n=== Step 2: Handling utils/mod.rs ==="
if [ -f "$BACKUP/utils/mod.rs" ]; then
    echo "Creating safe version of utils/mod.rs..."
    cp "src/utils/mod.rs" "src/utils/mod.rs.bak"
    
    # Create a version without serde_yaml issues
    cat > /tmp/utils_safe.rs << 'EOF'
pub mod error;
pub mod validation;
pub mod logger;
pub mod cryptography;
pub mod datetime;

use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

// Removed serde_yaml functions to fix compilation
// Original file had issues with serde_yaml dependency
EOF
    
    cp /tmp/utils_safe.rs "src/utils/mod.rs"
    
    if test_compilation; then
        echo "Safe version works!"
    else
        echo "Keeping minimal version"
        cp "src/utils/mod.rs.bak" "src/utils/mod.rs"
    fi
fi

# Step 3: Restore application DTOs and validators
echo -e "\n=== Step 3: Restoring application modules ==="
APP_FILES=(
    "application/dto/mod.rs"
    "application/dto/requests/mod.rs"
    "application/dto/responses/mod.rs"
    "application/validators/mod.rs"
    "application/validators/text_validator.rs"
    "application/validators/member_validator.rs"
    "application/validators/donation_validator.rs"
    "application/validators/vote_validator.rs"
)

for file in "${APP_FILES[@]}"; do
    echo -e "\n--- $file ---"
    if [ -f "$BACKUP/$file" ]; then
        if [ -f "src/$file" ]; then
            cp "src/$file" "src/$file.bak"
        fi
        
        mkdir -p "src/$(dirname "$file")"
        cp "$BACKUP/$file" "src/$file"
        echo "Copied"
        
        if ! test_compilation; then
            echo "Creating minimal version..."
            if [ -f "src/$file.bak" ]; then
                cp "src/$file.bak" "src/$file"
            else
                echo "// Module placeholder" > "src/$file"
            fi
            cp "$BACKUP/$file" "problematic_files/$file"
        fi
    else
        echo "Not found in backup"
    fi
done

# Step 4: Restore domain modules
echo -e "\n=== Step 4: Restoring domain modules ==="
# First, let's see what domain files exist in backup
DOMAIN_FILES=$(find "$BACKUP/domain" -name "*.rs" 2>/dev/null || true)

if [ -n "$DOMAIN_FILES" ]; then
    echo "Found domain files. Let's restore them carefully..."
    
    # Create domain directory structure
    mkdir -p src/domain/{models,value_objects,aggregates,repositories,enums}
    
    # Restore enums first (usually simple)
    if [ -f "$BACKUP/domain/enums/mod.rs" ]; then
        echo -e "\n--- domain/enums/mod.rs ---"
        cp "$BACKUP/domain/enums/mod.rs" "src/domain/enums/mod.rs"
        if ! test_compilation; then
            echo "Creating placeholder..."
            echo "// Enums placeholder" > "src/domain/enums/mod.rs"
            cp "$BACKUP/domain/enums/mod.rs" "problematic_files/domain/enums/mod.rs"
        fi
    fi
    
    # Restore models
    if [ -d "$BACKUP/domain/models" ]; then
        echo -e "\n--- Restoring models ---"
        for model_file in "$BACKUP/domain/models"/*.rs; do
            if [ -f "$model_file" ]; then
                filename=$(basename "$model_file")
                echo "- $filename"
                cp "$model_file" "src/domain/models/$filename"
                
                if ! test_compilation; then
                    echo "  Skipping (caused error)"
                    cp "$model_file" "problematic_files/domain/models/$filename"
                    # Remove problematic file
                    rm "src/domain/models/$filename"
                    # Create empty file
                    touch "src/domain/models/$filename"
                fi
            fi
        done
    fi
else
    echo "No domain files found in backup"
fi

# Step 5: Summary
echo -e "\n=== Summary ==="
echo "Restoration completed!"
echo ""
echo "✅ Compiling files restored to src/"
echo "❌ Problematic files saved to problematic_files/"
echo ""
echo "Next steps:"
echo "1. Review problematic files for issues:"
echo "   ls -la problematic_files/"
echo ""
echo "2. Test the current build:"
echo "   cargo check"
echo ""
echo "3. If it compiles, you can start restoring more complex modules:"
echo "   - application/services/"
echo "   - application/queries/"
echo "   - application/commands/"
echo "   - infrastructure/"
echo ""
echo "4. Run this script again with more files if needed"
