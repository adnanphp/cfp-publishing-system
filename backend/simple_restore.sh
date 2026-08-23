#!/bin/bash

echo "Simple Restoration Script"
echo "========================"
echo ""

BACKUP="./src.backup.1769027748"
LOG_FILE="restoration.log"

echo "Restoration started: $(date)" > "$LOG_FILE"

# Function to restore a module
restore_module() {
    local module_path=$1
    local module_name=$(basename "$module_path")
    
    echo "" >> "$LOG_FILE"
    echo "=== Attempting to restore: $module_path ===" >> "$LOG_FILE"
    
    # Check if module exists in backup
    if [ ! -d "$BACKUP/$module_path" ]; then
        echo "  ❌ Not found in backup" | tee -a "$LOG_FILE"
        return 1
    fi
    
    # Create target directory
    mkdir -p "src/$module_path"
    
    # Count files
    file_count=$(find "$BACKUP/$module_path" -name "*.rs" | wc -l)
    echo "  Found $file_count .rs files" | tee -a "$LOG_FILE"
    
    # Copy files
    echo "  Copying files..." | tee -a "$LOG_FILE"
    cp "$BACKUP/$module_path"/*.rs "src/$module_path"/ 2>/dev/null || true
    
    # Test compilation
    echo "  Testing compilation..." | tee -a "$LOG_FILE"
    if cargo check --quiet 2>> "$LOG_FILE"; then
        echo "  ✅ SUCCESS: Module compiles" | tee -a "$LOG_FILE"
        return 0
    else
        echo "  ❌ FAILED: Compilation errors" | tee -a "$LOG_FILE"
        # Remove problematic files
        rm -f "src/$module_path"/*.rs 2>/dev/null
        # Create placeholder
        echo "// Placeholder for $module_name" > "src/$module_path/mod.rs"
        return 1
    fi
}

echo "Starting restoration process..."
echo ""

# List of modules to try (in order of simplicity)
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

success=0
failed=0

for module in "${MODULES[@]}"; do
    echo "--- $module ---"
    if restore_module "$module"; then
        ((success++))
    else
        ((failed++))
    fi
    echo ""
done

echo "================================="
echo "Restoration Complete!"
echo "✅ Successfully restored: $success modules"
echo "❌ Failed to restore: $failed modules"
echo ""
echo "Check $LOG_FILE for details"
echo ""
echo "Next: Run 'cargo check' to verify overall compilation"
