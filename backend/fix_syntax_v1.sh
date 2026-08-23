#!/bin/bash

echo "Fixing syntax errors..."
echo "======================="

# 1. Fix author.rs duplicate Ok line
echo -e "\n1. Fixing author.rs..."
if [ -f "src/domain/models/author.rs" ]; then
    echo "Removing duplicate Ok line in author.rs..."
    # Show lines around the error
    echo "Lines 40-50:"
    sed -n '40,50p' src/domain/models/author.rs
    
    # Fix: remove line 44 (duplicate Ok line)
    sed -i '44d' src/domain/models/author.rs
    # Add semicolon to line 43
    sed -i '43s/$/;/' src/domain/models/author.rs
    
    echo "After fix:"
    sed -n '40,45p' src/domain/models/author.rs
fi

# 2. Fix text.rs missing semicolon
echo -e "\n2. Fixing text.rs..."
if [ -f "src/domain/models/text.rs" ]; then
    echo "Adding missing semicolon in text.rs..."
    # Show lines around the error
    echo "Lines 75-85:"
    sed -n '75,85p' src/domain/models/text.rs
    
    # Add semicolon to line 79
    sed -i '79s/$/;/' src/domain/models/text.rs
    
    echo "After fix:"
    sed -n '75,85p' src/domain/models/text.rs
fi

# 3. Fix logger.rs extra closing brace
echo -e "\n3. Fixing logger.rs..."
if [ -f "src/utils/logger.rs" ]; then
    echo "Removing extra closing brace in logger.rs..."
    # Show lines around the error
    echo "Lines 65-75:"
    sed -n '65,75p' src/utils/logger.rs
    
    # Remove line 71 (extra closing brace)
    sed -i '71d' src/utils/logger.rs
    
    echo "After fix:"
    sed -n '65,75p' src/utils/logger.rs
fi

# 4. Also fix the sed command issue from previous script
echo -e "\n4. Cleaning up validator attributes properly..."
for file in src/domain/models/author.rs src/domain/models/text.rs; do
    if [ -f "$file" ]; then
        echo "Cleaning $file..."
        # Use perl for more reliable regex
        perl -i -pe 's/#\[validate\([^)]*\)\]//g' "$file"
        perl -i -pe 's/use validator::Validate;//g' "$file"
        perl -i -pe 's/, Validate//g' "$file"
        perl -i -pe 's/#\[derive\((.*), Validate(.*)\)/#[derive($1$2)/g' "$file"
        perl -i -pe 's/#\[derive\((.*)Validate(.*)\)/#[derive($1$2)/g' "$file"
    fi
done

# 5. Test compilation
echo -e "\n5. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo -e "\n✅ SUCCESS! Project compiles!"
    
    echo -e "\nYou now have a compiling project!"
    echo ""
    echo "Next steps:"
    echo "1. List remaining modules to restore:"
    echo "   find ./src.backup.1769027748 -name '*.rs' | grep -E '(services|queries|commands|infrastructure|api)' | sort"
    echo ""
    echo "2. Restore modules one by one:"
    echo "   cp ./src.backup.1769027748/application/services/mod.rs src/application/services/"
    echo "   cargo check"
    echo ""
    echo "3. Continue until all modules are restored"
    echo ""
    echo "4. Test with: cargo build"
else
    echo -e "\n❌ Still have compilation errors:"
    cargo check 2>&1 | grep -B 2 -A 2 "error\[E"
fi
