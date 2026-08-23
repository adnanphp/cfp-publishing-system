#!/bin/bash

echo "Analyzing backup structure..."
echo "============================="

BACKUP="./src.backup.1769027748"

# Show directory structure
echo "Backup directory structure:"
find "$BACKUP" -type d | sort

echo ""
echo "File types found:"
find "$BACKUP" -name "*.rs" | xargs -I {} basename {} | sort | uniq -c | sort -rn | head -20

echo ""
echo "Let's check what modules we should restore:"

# Check for common module patterns
if [ -d "$BACKUP/application/services" ]; then
    echo "✅ Found: application/services"
    ls "$BACKUP/application/services/"*.rs 2>/dev/null | wc -l
fi

if [ -d "$BACKUP/api/handlers" ]; then
    echo "✅ Found: api/handlers"
    ls "$BACKUP/api/handlers/"*.rs 2>/dev/null | wc -l
fi

# Create a test restoration for one file
echo ""
echo "Testing restoration of a single file..."
TEST_FILE=$(find "$BACKUP" -name "*.rs" | head -1)
if [ -n "$TEST_FILE" ]; then
    echo "Testing with: $(basename "$TEST_FILE")"
    
    # Create a test
    cp "$TEST_FILE" /tmp/test_restore.rs 2>/dev/null
    echo "File copied for testing"
fi

echo ""
echo "Recommendation: Let's restore module by module manually."
echo "First, let's see what's actually in your backup directories:"

# Check each potential module directory
for dir in application api infrastructure domain utils; do
    if [ -d "$BACKUP/$dir" ]; then
        echo ""
        echo "=== $dir/ ==="
        find "$BACKUP/$dir" -name "*.rs" | head -5 | xargs -I {} basename {}
        count=$(find "$BACKUP/$dir" -name "*.rs" | wc -l)
        echo "Total: $count files"
    fi
done

echo ""
echo "To restore, I recommend:"
echo "1. First clean up unused imports:"
echo "   cargo fix --lib -p cfp-backend --allow-dirty"
echo ""
echo "2. Save current working state:"
echo "   git add . && git commit -m 'Working base'"
echo ""
echo "3. Restore ONE directory at a time:"
echo "   Example for application/services:"
echo "   mkdir -p src/application/services"
echo "   cp $BACKUP/application/services/*.rs src/application/services/ 2>/dev/null || echo 'No files'"
echo "   cargo check"
echo ""
echo "4. If it compiles, commit and move to next"
echo "5. If not, fix errors or skip that module"
