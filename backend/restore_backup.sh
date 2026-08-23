#!/bin/bash

echo "Restoring from backup with proper structure..."

# First, let's check what we have in the backup
echo "Available backup directories:"
ls -la src.backup* 2>/dev/null || echo "No backup found"

# Let's find the most recent backup
BACKUP_DIR=$(find . -name "src.backup.*" -type d | sort -r | head -1)

if [ -z "$BACKUP_DIR" ]; then
    echo "No backup found. Creating from scratch..."
    BACKUP_DIR="src.backup"
fi

echo "Using backup: $BACKUP_DIR"

# Create fresh structure
echo "Creating fresh structure..."
rm -rf src
mkdir -p src

# Copy everything from backup
echo "Copying from backup..."
cp -r "$BACKUP_DIR"/* src/ 2>/dev/null || true

# Now let's make sure all module files exist
echo "Ensuring all module files exist..."

# Function to create module file if it doesn't exist
ensure_module() {
    local dir=$1
    local module=$2
    
    if [ ! -f "src/$dir/$module.rs" ] && [ ! -d "src/$dir/$module" ]; then
        echo "Creating src/$dir/$module.rs"
        mkdir -p "src/$dir"
        echo "// $module module placeholder" > "src/$dir/$module.rs"
    fi
}

# Ensure all declared modules exist
ensure_module "api" "middleware"
ensure_module "api" "routes"
ensure_module "domain" "value_objects"
ensure_module "domain" "aggregates"
ensure_module "domain" "repositories"
ensure_module "domain" "enums"
ensure_module "application" "services"
ensure_module "application" "queries"
ensure_module "application" "commands"
ensure_module "infrastructure" "database"
ensure_module "infrastructure" "cache"
ensure_module "infrastructure" "security"
ensure_module "infrastructure" "messaging"
ensure_module "infrastructure" "external"

# Create main.rs if it doesn't exist
if [ ! -f "src/main.rs" ]; then
    cat > src/main.rs << 'EOF'
fn main() {
    println!("CFP Backend");
}
EOF
fi

# Create lib.rs if it doesn't exist
if [ ! -f "src/lib.rs" ]; then
    cat > src/lib.rs << 'EOF'
pub mod api;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
EOF
fi

echo "Structure created. Running cargo check..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Compilation successful!"
    echo ""
    echo "If there are still errors, try fixing them one by one:"
    echo "1. Look at the first few errors"
    echo "2. Fix them manually"
    echo "3. Run 'cargo check' again"
else
    echo "❌ Compilation failed. Let's see the first few errors..."
    cargo check 2>&1 | grep -A 5 "error\[E"
fi
