#!/bin/bash

BACKUP="./src.backup.1769027748"
RESTORE_LOG="restore_log.txt"

echo "Restoration started at: $(date)" > "$RESTORE_LOG"
echo "=================================" >> "$RESTORE_LOG"

restore_module() {
    local module=$1
    local source_dir="$BACKUP/$module"
    local target_dir="src/$module"
    
    echo "" >> "$RESTORE_LOG"
    echo "=== Restoring: $module ===" >> "$RESTORE_LOG"
    
    if [ ! -d "$source_dir" ]; then
        echo "Source directory not found: $source_dir" | tee -a "$RESTORE_LOG"
        return 1
    fi
    
    # Create target directory
    mkdir -p "$target_dir"
    
    # Count files before copy
    local file_count=$(find "$source_dir" -name "*.rs" | wc -l)
    echo "Found $file_count .rs files" >> "$RESTORE_LOG"
    
    # Copy files
    cp -r "$source_dir"/*.rs "$target_dir"/ 2>/dev/null || true
    
    # Test compilation
    echo "Testing compilation..." >> "$RESTORE_LOG"
    if cargo check --quiet 2>> "$RESTORE_LOG"; then
        echo "✅ $module: SUCCESS (compiles)" | tee -a "$RESTORE_LOG"
        return 0
    else
        echo "❌ $module: FAILED (compilation errors)" | tee -a "$RESTORE_LOG"
        # Show first error
        cargo check 2>&1 | grep -A 2 "error\[E" | head -5 >> "$RESTORE_LOG"
        
        # Revert - remove copied files
        rm -f "$target_dir"/*.rs 2>/dev/null
        # Keep directory but create empty mod.rs
        echo "// Placeholder - restoration failed" > "$target_dir/mod.rs"
        return 1
    fi
}

# Restore in recommended order
MODULES=(
    "application/services"
    "application/queries"
    "application/commands"
    "infrastructure/database"
    "infrastructure/cache"
    "infrastructure/security"
    "infrastructure/messaging"
    "infrastructure/external"
    "api/handlers"
    "api/routes"
    "api/middleware"
)

echo "Starting gradual restoration..." | tee -a "$RESTORE_LOG"
echo "===============================" | tee -a "$RESTORE_LOG"

success_count=0
fail_count=0

for module in "${MODULES[@]}"; do
    if restore_module "$module"; then
        ((success_count++))
    else
        ((fail_count++))
    fi
done

echo "" | tee -a "$RESTORE_LOG"
echo "=================================" | tee -a "$RESTORE_LOG"
echo "Restoration completed!" | tee -a "$RESTORE_LOG"
echo "✅ Successfully restored: $success_count modules" | tee -a "$RESTORE_LOG"
echo "❌ Failed to restore: $fail_count modules" | tee -a "$RESTORE_LOG"
echo "" | tee -a "$RESTORE_LOG"
echo "Check $RESTORE_LOG for details"
echo "" | tee -a "$RESTORE_LOG"
echo "Next steps:" | tee -a "$RESTORE_LOG"
echo "1. Review failed modules in the log" | tee -a "$RESTORE_LOG"
echo "2. Manually fix any compilation errors" | tee -a "$RESTORE_LOG"
echo "3. Run: cargo build" | tee -a "$RESTORE_LOG"
echo "4. Run: cargo test (if you have tests)" | tee -a "$RESTORE_LOG"
