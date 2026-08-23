#!/bin/bash

echo "Setting up database..."

# Check if PostgreSQL is running
if ! pg_isready -h localhost; then
    echo "PostgreSQL is not running. Starting..."
    sudo service postgresql start
    sleep 2
fi

# Check if database exists
if ! psql -h localhost -U adnan -lqt | cut -d \| -f 1 | grep -qw cfp_db; then
    echo "Creating database cfp_db..."
    createdb -h localhost -U adnan cfp_db
fi

# Drop and recreate with new schema
echo "Dropping existing tables..."
psql -h localhost -U adnan -d cfp_db -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;" 2>/dev/null || true

echo "Creating tables from schema..."
psql -h localhost -U adnan -d cfp_db -f create_final_tables.sql

echo "Granting permissions to adnan user..."
psql -h localhost -U adnan -d cfp_db << 'GRANTEOF'
GRANT ALL PRIVILEGES ON DATABASE cfp_db TO adnan;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO adnan;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO adnan;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO adnan;
GRANTEOF

echo "Database setup complete!"
echo ""
echo "Test connection: psql -h localhost -U adnan -d cfp_db -c 'SELECT 1;'"
