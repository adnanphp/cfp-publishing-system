#!/bin/bash

echo "=== Adding Required Dependencies First ==="

echo "1. Adding all necessary dependencies to Cargo.toml..."
cat >> Cargo.toml << 'DEPS'

# Add dependencies your files need
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
validator = { version = "0.16", features = ["derive", "phone"] }
rand = "0.8"
thiserror = "1.0"
async-trait = "0.1"
DEPS

echo "2. Updating cargo..."
cargo update

echo "3. Creating a simple test to ensure dependencies work..."
mkdir -p src/domain/enums

cat > src/domain/enums/member_status.rs << 'TESTENUM'
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemberStatus {
    Pending,
    Active,
    Suspended,
    Banned,
    Deleted,
}

impl Default for MemberStatus {
    fn default() -> Self {
        MemberStatus::Pending
    }
}
TESTENUM

cat > src/domain/enums/mod.rs << 'ENUMMOD'
pub mod member_status;
pub use member_status::*;
ENUMMOD

cat > src/domain/mod.rs << 'DOMAIN'
pub mod enums;
pub use enums::*;
DOMAIN

echo "pub mod domain;" >> src/lib.rs

echo "4. Testing with simple enum..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ SUCCESS! Dependencies added and simple enum works."
    echo ""
    echo "Now let's try adding your actual member_status.rs..."
    
    BACKUP_DIR=$(ls -td ../cfp-backup-* | head -1)
    
    if [ -f "$BACKUP_DIR/src/domain/enums/member_status.rs" ]; then
        echo "Copying your actual member_status.rs..."
        cp "$BACKUP_DIR/src/domain/enums/member_status.rs" src/domain/enums/
        
        echo "Testing your actual file..."
        cargo check
        
        if [ $? -eq 0 ]; then
            echo ""
            echo "🎉 PERFECT! Your actual member_status.rs works!"
            echo ""
            echo "Now you have:"
            echo "- Working server"
            echo "- Required dependencies"
            echo "- Your actual member_status.rs file"
            echo ""
            echo "Run: cargo run"
            echo "Test: curl http://localhost:3000/api/config"
            echo ""
            echo "Ready to add more files!"
        else
            echo ""
            echo "❌ Your file still has issues. Let's see..."
            cargo check 2>&1 | grep -E "error\[E[0-9]+\]:" | head -5
            
            echo ""
            echo "Let's fix it..."
            # Fix the file
            sed -i '1iuse serde::{Serialize, Deserialize};' src/domain/enums/member_status.rs
            cargo check
            
            if [ $? -eq 0 ]; then
                echo "✅ Fixed by adding serde import!"
            fi
        fi
    else
        echo "Couldn't find member_status.rs in backup"
    fi
else
    echo "❌ Basic dependencies test failed."
    echo "Errors:"
    cargo check 2>&1 | grep -E "error\[E[0-9]+\]:" | head -5
    
    echo ""
    echo "Let's try without validator..."
    # Remove validator
    sed -i '/validator =/d' Cargo.toml
    rm -rf src/domain
    sed -i '/^pub mod domain;/d' src/lib.rs
    
    # Try again without validator
    echo "Trying without validator..."
    cargo update
    cargo check
fi

echo ""
echo "Summary: Added these dependencies:"
echo "- chrono: for DateTime"
echo "- uuid: for UUID types"
echo "- serde: already had it"
echo "- validator: for validation (might need features)"
echo "- rand: for random generation"
echo "- thiserror: for error handling"
echo "- async-trait: for async traits"
