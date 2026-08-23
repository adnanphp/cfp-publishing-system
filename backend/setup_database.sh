#!/bin/bash

echo "Setting up database..."

# Check if database exists
if ! psql -h localhost -U adnan -d cfp_db -c "SELECT 1" >/dev/null 2>&1; then
    echo "Database cfp_db doesn't exist. Creating..."
    createdb -h localhost -U adnan cfp_db
fi

# Run the schema creation
echo "Creating tables..."
psql -h localhost -U adnan -d cfp_db -f create_correct_tables.sql

echo "Database setup complete!"
