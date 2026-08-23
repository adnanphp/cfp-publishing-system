#!/bin/bash
# add-backend-source.sh - Add only backend source files

echo "📦 Adding Backend Source Files Only"
echo "===================================="

# 1. Remove the submodule reference
echo "1️⃣ Removing submodule reference..."
git rm --cached backend
rm -rf backend/.git 2>/dev/null
echo "   ✅ Removed"

# 2. Add only essential files
echo ""
echo "2️⃣ Adding essential backend files..."

# Add Cargo files
git add backend/Cargo.toml backend/Cargo.lock 2>/dev/null && echo "   ✅ Added Cargo files"

# Add actual source code (NOT backups)
git add backend/src/ 2>/dev/null && echo "   ✅ Added src/"

# Add migrations if they exist
[ -d "backend/migrations" ] && git add backend/migrations/ && echo "   ✅ Added migrations/"

# Add config if it exists
[ -d "backend/config" ] && git add backend/config/ && echo "   ✅ Added config/"

# Add configuration files
[ -f "backend/.env.example" ] && git add backend/.env.example && echo "   ✅ Added .env.example"

# 3. Check what's staged
echo ""
echo "3️⃣ Staged backend files:"
git diff --cached --name-only | grep "^backend/" | head -20

# 4. Show size
echo ""
echo "4️⃣ Size of staged backend files:"
git diff --cached --name-only | grep "^backend/" | xargs du -ch 2>/dev/null | tail -1

# 5. Commit
echo ""
echo "5️⃣ Committing..."
git commit -m "Add backend source code

- Rust source files in backend/src/
- Cargo.toml and Cargo.lock
- Database migrations
- Configuration files
- Excludes backup folders, target/, and build artifacts"

# 6. Push
echo ""
echo "6️⃣ Pushing to GitHub..."
git push origin main

echo ""
echo "✅ Backend source code pushed!"
echo "📁 View at: https://github.com/adnanphp/cfp-publishing-system/tree/main/backend"
