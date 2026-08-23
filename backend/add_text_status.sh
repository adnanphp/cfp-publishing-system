#!/bin/bash

echo "=== Adding text_status.rs ==="

echo "1. Getting text_status.rs from backup..."
BACKUP_DIR=$(ls -td ../cfp-backup-* | head -1)

if [ -f "$BACKUP_DIR/src/domain/enums/text_status.rs" ]; then
    echo "Copying text_status.rs..."
    cp "$BACKUP_DIR/src/domain/enums/text_status.rs" src/domain/enums/
    
    echo "2. Fixing sqlx imports if present..."
    # Remove sqlx imports and attributes
    sed -i '/^use sqlx/d' src/domain/enums/text_status.rs
    sed -i '/^#\[sqlx/d' src/domain/enums/text_status.rs
    
    # Ensure serde import is at top
    if ! grep -q "use serde" src/domain/enums/text_status.rs; then
        sed -i '1iuse serde::{Deserialize, Serialize};' src/domain/enums/text_status.rs
    fi
    
    echo "3. Updating mod.rs..."
    echo "pub mod text_status;" >> src/domain/enums/mod.rs
    echo "pub use text_status::*;" >> src/domain/enums/mod.rs
    
    echo "4. Testing compilation..."
    cargo check
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✅ SUCCESS! text_status.rs added and compiles."
        echo ""
        echo "Now you have:"
        echo "- member_status.rs"
        echo "- text_status.rs"
        echo "- All working together"
        echo ""
        echo "Want to add one more? (y/n)"
        read -p "Add charity_status.rs? " add_more
        
        if [ "$add_more" = "y" ]; then
            echo "Adding charity_status.rs..."
            cp "$BACKUP_DIR/src/domain/enums/charity_status.rs" src/domain/enums/
            
            # Fix sqlx imports
            sed -i '/^use sqlx/d' src/domain/enums/charity_status.rs
            sed -i '/^#\[sqlx/d' src/domain/enums/charity_status.rs
            
            # Add serde import if missing
            if ! grep -q "use serde" src/domain/enums/charity_status.rs; then
                sed -i '1iuse serde::{Deserialize, Serialize};' src/domain/enums/charity_status.rs
            fi
            
            echo "pub mod charity_status;" >> src/domain/enums/mod.rs
            echo "pub use charity_status::*;" >> src/domain/enums/mod.rs
            
            cargo check
            
            if [ $? -eq 0 ]; then
                echo "✅ charity_status.rs added successfully!"
            else
                echo "❌ charity_status.rs has issues. Removing..."
                rm src/domain/enums/charity_status.rs
                sed -i '/pub mod charity_status;/d' src/domain/enums/mod.rs
                sed -i '/pub use charity_status::\*;/d' src/domain/enums/mod.rs
                cargo check
            fi
        fi
        
        echo ""
        echo "🎉 Great progress! Now let's test the server..."
        echo ""
        echo "Stop any running server (Ctrl+C)"
        echo "Then run: cargo run"
        echo "Test: curl http://localhost:3000/api/config"
        echo ""
        echo "Your domain/enums module now has actual working files!"
        
    else
        echo "❌ text_status.rs has issues. Let's create a simple version..."
        
        cat > src/domain/enums/text_status.rs << 'TEXTSTATUS'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Published,
    Rejected,
    Archived,
}

impl Default for TextStatus {
    fn default() -> Self {
        TextStatus::Draft
    }
}

impl TextStatus {
    pub fn can_be_edited(&self) -> bool {
        matches!(self, TextStatus::Draft | TextStatus::Rejected)
    }
    
    pub fn is_visible_to_public(&self) -> bool {
        matches!(self, TextStatus::Published)
    }
}
TEXTSTATUS
        
        cargo check
        echo "✅ Created simple text_status.rs"
    fi
    
else
    echo "❌ text_status.rs not found in backup. Creating simple version..."
    
    cat > src/domain/enums/text_status.rs << 'TEXTSTATUS'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Published,
    Rejected,
    Archived,
}

impl Default for TextStatus {
    fn default() -> Self {
        TextStatus::Draft
    }
}
TEXTSTATUS
    
    echo "pub mod text_status;" >> src/domain/enums/mod.rs
    echo "pub use text_status::*;" >> src/domain/enums/mod.rs
    
    cargo check
    echo "✅ Created text_status.rs"
fi

echo ""
echo "Summary of domain/enums module:"
ls -la src/domain/enums/*.rs | awk '{print "  " $9}'
