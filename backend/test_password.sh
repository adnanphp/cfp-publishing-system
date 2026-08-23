#!/bin/bash

echo "🔑 Testing PostgreSQL password..."

# Read password from .env file
if [ -f .env ]; then
    db_url=$(grep DATABASE_URL .env | cut -d= -f2)
    echo "Found DATABASE_URL: ${db_url//:[^:]*@/:****@}"
    
    # Extract password
    password=$(echo "$db_url" | sed -n 's/.*:\/\/[^:]*:\([^@]*\)@.*/\1/p')
    
    echo "Testing password..."
    psql "postgresql://adnan:${password}@localhost:5432/cfp_db" -c "SELECT 'Test' as result;" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo "✅ Password in .env file WORKS!"
    else
        echo "❌ Password in .env file FAILS"
    fi
else
    echo "❌ No .env file found"
fi

echo ""
echo "💡 Manual test:"
read -sp "Enter your actual PostgreSQL password: " manual_password
echo ""

psql "postgresql://adnan:${manual_password}@localhost:5432/cfp_db" -c "SELECT 'Manual test' as result;" 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ Manual password WORKS!"
    echo ""
    echo "📋 Update .env file with:"
    echo "DATABASE_URL=postgresql://adnan:${manual_password}@localhost:5432/cfp_db"
else
    echo "❌ Manual password also FAILS"
    echo ""
    echo "💡 You need to reset the password:"
    echo "sudo -u postgres psql -c \"ALTER USER adnan WITH PASSWORD 'new_password123';\""
fi
