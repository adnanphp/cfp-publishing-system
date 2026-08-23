#!/bin/bash

echo "Careful File-by-File Restoration"
echo "================================"

BACKUP="./src.backup.1769027748"
RESTORE_LOG="careful_restore.log"

echo "Starting careful restoration: $(date)" > "$RESTORE_LOG"

restore_file() {
    local source_file=$1
    local target_dir=$2
    
    echo "" >> "$RESTORE_LOG"
    echo "=== Testing: $(basename "$source_file") ===" >> "$RESTORE_LOG"
    
    # Backup original if it exists
    local target_file="$target_dir/$(basename "$source_file")"
    if [ -f "$target_file" ]; then
        cp "$target_file" "$target_file.backup"
    fi
    
    # Copy the file
    cp "$source_file" "$target_file"
    
    # Test compilation
    if cargo check --quiet 2>> "$RESTORE_LOG"; then
        echo "✅ $(basename "$source_file"): OK" | tee -a "$RESTORE_LOG"
        return 0
    else
        echo "❌ $(basename "$source_file"): FAILED" | tee -a "$RESTORE_LOG"
        # Restore backup if we had one
        if [ -f "$target_file.backup" ]; then
            cp "$target_file.backup" "$target_file"
        else
            # Create minimal placeholder
            echo "// Placeholder - original caused errors" > "$target_file"
        fi
        # Save problematic file
        mkdir -p "problematic_files/$(dirname "$target_file")"
        cp "$source_file" "problematic_files/$target_file"
        return 1
    fi
}

# Start with utils directory
echo "Starting with utils directory..."
UTILS_FILES=(
    "validation.rs"
    "logger.rs"
    "cryptography.rs"
    "datetime.rs"
    "error.rs"
)

success=0
failed=0

for file in "${UTILS_FILES[@]}"; do
    if [ -f "$BACKUP/utils/$file" ]; then
        echo "Testing: $file"
        if restore_file "$BACKUP/utils/$file" "src/utils"; then
            ((success++))
        else
            ((failed++))
        fi
    else
        echo "⚠️  File not found: $file"
    fi
done

echo ""
echo "================================="
echo "Utils restoration complete:"
echo "✅ Success: $success files"
echo "❌ Failed: $failed files"
echo ""
echo "Check $RESTORE_LOG for details"
echo ""
echo "Current compilation status:"
cargo check --quiet && echo "✅ Project compiles" || echo "❌ Project has errors"
