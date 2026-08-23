#!/bin/bash
# security-check.sh - Check for sensitive data

echo "🔍 Security Check"
echo "================"

# Check for real passwords (look for patterns)
echo "1️⃣ Checking for real password patterns..."
git grep -E "postgresql://[^:]+:[^@]+@" -- "*.env*" 2>/dev/null | grep -v ".example" || echo "   ✅ No real database passwords found"

# Check for secret keys
echo ""
echo "2️⃣ Checking for secret keys..."
git grep -E "secret.*=.*[A-Za-z0-9]{32,}" -- "*.env*" 2>/dev/null | grep -v ".example" || echo "   ✅ No real secret keys found"

# Check if .env files are tracked
echo ""
echo "3️⃣ Checking if .env files are tracked..."
git ls-files | grep -E "\.env$" || echo "   ✅ No .env files tracked"

echo ""
echo "✅ Security check complete!"
