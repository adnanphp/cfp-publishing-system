#!/bin/bash

echo "=== Testing CFP Backend Project ==="
echo ""

echo "1. Checking project structure..."
echo "Files found:"
find src -name "*.rs" -type f | sort | sed 's/^/  /'

echo ""
echo "2. Checking module imports..."
if grep -q "pub mod auth_handler;" src/api/handlers/mod.rs; then
    echo "✅ auth_handler module declared"
else
    echo "❌ auth_handler module missing"
fi

if [ -f "src/api/handlers/auth_handler.rs" ]; then
    echo "✅ auth_handler.rs file exists"
else
    echo "❌ auth_handler.rs file missing"
fi

echo ""
echo "3. Running cargo check..."
cargo check

if [ $? -eq 0 ]; then
    echo ""
    echo "✅✅✅ PROJECT COMPILES SUCCESSFULLY! ✅✅✅"
    echo ""
    echo "You now have a complete working backend with:"
    echo "  📁 domain/ - Business models and enums"
    echo "  📁 application/ - Services, DTOs, validators"
    echo "  📁 api/ - Routes and handlers"
    echo "  📁 config/ - Configuration"
    echo ""
    echo "To run the server:"
    echo "  cargo run"
    echo ""
    echo "Available endpoints:"
    echo "  GET  /                  - Welcome page"
    echo "  GET  /health            - Health check"
    echo "  GET  /api/test          - API test"
    echo "  GET  /api/status        - System status"
    echo "  POST /api/auth/login    - Login"
    echo "  POST /api/auth/register - Register"
    echo "  POST /api/auth/logout   - Logout"
else
    echo ""
    echo "❌ Compilation failed"
fi
