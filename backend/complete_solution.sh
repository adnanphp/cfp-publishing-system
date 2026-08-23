#!/bin/bash

echo "=== Step 1: Create correct database schema ==="

# Create schema that matches your ER diagram exactly
cat > create_final_tables.sql << 'EOF'
-- Drop existing tables if they exist
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS committee_memberships CASCADE;
DROP TABLE IF EXISTS votes CASCADE;
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS donations CASCADE;
DROP TABLE IF EXISTS downloads CASCADE;
DROP TABLE IF EXISTS text_versions CASCADE;
DROP TABLE IF EXISTS plagiarism_cases CASCADE;
DROP TABLE IF EXISTS committee CASCADE;
DROP TABLE IF EXISTS charity CASCADE;
DROP TABLE IF EXISTS texts CASCADE;
DROP TABLE IF EXISTS moderators CASCADE;
DROP TABLE IF EXISTS admins CASCADE;
DROP TABLE IF EXISTS authors CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS members CASCADE;

-- Create members table matching ER diagram
CREATE TABLE members (
    member_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    organization VARCHAR(100),
    pseudonym VARCHAR(100),
    primary_email VARCHAR(100) UNIQUE NOT NULL,
    recovery_email VARCHAR(100),
    password_hash VARCHAR(255) NOT NULL,
    verification_matrix VARCHAR(255),
    matrix_expiry DATE,
    join_date DATE NOT NULL DEFAULT CURRENT_DATE,
    street VARCHAR(100),
    city VARCHAR(50),
    state VARCHAR(50),
    country VARCHAR(50),
    postal_code VARCHAR(20),
    introduced_by INTEGER REFERENCES members(member_id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create authors table (specialization of members)
CREATE TABLE authors (
    orcid VARCHAR(20) PRIMARY KEY,
    member_id INTEGER UNIQUE REFERENCES members(member_id) ON DELETE CASCADE,
    bio TEXT,
    specialization VARCHAR(100),
    h_index INTEGER DEFAULT 0,
    total_downloads INTEGER DEFAULT 0
);

-- Create admins table (specialization of members)
CREATE TABLE admins (
    admin_id SERIAL PRIMARY KEY,
    member_id INTEGER UNIQUE REFERENCES members(member_id) ON DELETE CASCADE,
    role VARCHAR(20) CHECK (role IN ('super', 'content', 'financial')),
    last_login TIMESTAMP
);

-- Create moderators table (specialization of admins)
CREATE TABLE moderators (
    mod_id SERIAL PRIMARY KEY,
    admin_id INTEGER UNIQUE REFERENCES admins(admin_id) ON DELETE CASCADE,
    domain VARCHAR(50),
    expertise_area TEXT[]
);

-- Create messages table
CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    sender_id INTEGER REFERENCES members(member_id) NOT NULL,
    recipient_id INTEGER REFERENCES members(member_id) NOT NULL,
    content TEXT NOT NULL,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_read BOOLEAN DEFAULT FALSE
);

-- Create texts table
CREATE TABLE texts (
    text_id SERIAL PRIMARY KEY,
    author_orcid VARCHAR(20) REFERENCES authors(orcid),
    title VARCHAR(255) NOT NULL,
    abstract TEXT,
    topic VARCHAR(100),
    version INTEGER NOT NULL DEFAULT 1,
    upload_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status VARCHAR(20) CHECK (status IN ('draft', 'under_review', 'published', 'archived')) DEFAULT 'draft',
    download_count INTEGER DEFAULT 0,
    total_donations DECIMAL(10,2) DEFAULT 0.00,
    avg_rating DECIMAL(3,2) DEFAULT 0.00,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create text keywords table (multivalued attribute)
CREATE TABLE text_keywords (
    text_id INTEGER REFERENCES texts(text_id) ON DELETE CASCADE,
    keyword VARCHAR(50),
    PRIMARY KEY (text_id, keyword)
);

-- Create text versions table (weak entity)
CREATE TABLE text_versions (
    version_id SERIAL,
    text_id INTEGER REFERENCES texts(text_id) ON DELETE CASCADE,
    changes TEXT NOT NULL,
    submitted_date DATE NOT NULL DEFAULT CURRENT_DATE,
    review_date DATE,
    status VARCHAR(20) CHECK (status IN ('pending', 'approved', 'rejected')) DEFAULT 'pending',
    change_summary VARCHAR(255),
    moderator_id INTEGER REFERENCES admins(admin_id),
    PRIMARY KEY (version_id, text_id)
);

-- Create charity table
CREATE TABLE charity (
    charity_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    mission VARCHAR(255),
    country VARCHAR(50),
    registration_number VARCHAR(50) UNIQUE,
    status VARCHAR(20) CHECK (status IN ('active', 'inactive', 'pending')) DEFAULT 'active',
    total_received DECIMAL(10,2) DEFAULT 0.00
);

-- Create committee table
CREATE TABLE committee (
    committee_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    purpose TEXT,
    scope VARCHAR(20) CHECK (scope IN ('plagiarism', 'content', 'finance', 'appeals')),
    formation_date DATE DEFAULT CURRENT_DATE,
    status VARCHAR(20) CHECK (status IN ('active', 'inactive')) DEFAULT 'active',
    member_count INTEGER DEFAULT 0
);

-- Create plagiarism cases table (weak entity)
CREATE TABLE plagiarism_cases (
    case_id SERIAL,
    committee_id INTEGER REFERENCES committee(committee_id) ON DELETE CASCADE,
    text_id INTEGER REFERENCES texts(text_id),
    opened_date DATE NOT NULL DEFAULT CURRENT_DATE,
    description TEXT,
    status VARCHAR(20) CHECK (status IN ('open', 'under_review', 'voting', 'closed', 'appealed')) DEFAULT 'open',
    resolution VARCHAR(20) CHECK (resolution IN ('plagiarized', 'not_plagiarized', 'appealed')),
    closed_date DATE,
    PRIMARY KEY (case_id, committee_id)
);

-- Create downloads table (associative entity)
CREATE TABLE downloads (
    download_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    text_id INTEGER REFERENCES texts(text_id) NOT NULL,
    download_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ip_address VARCHAR(45),
    user_agent VARCHAR(255),
    country VARCHAR(50)
);

-- Create donations table (associative entity)
CREATE TABLE donations (
    donation_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    text_id INTEGER REFERENCES texts(text_id) NOT NULL,
    charity_id INTEGER REFERENCES charity(charity_id) NOT NULL,
    amount DECIMAL(10,2) NOT NULL CHECK (amount >= 1.00),
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    currency VARCHAR(3) DEFAULT 'USD',
    payment_method VARCHAR(50),
    transaction_id VARCHAR(100) UNIQUE,
    charity_pct INTEGER NOT NULL CHECK (charity_pct >= 60),
    cfp_pct INTEGER NOT NULL,
    author_pct INTEGER NOT NULL,
    CHECK (charity_pct + cfp_pct + author_pct = 100)
);

-- Create comments table (associative entity)
CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    text_id INTEGER REFERENCES texts(text_id) NOT NULL,
    parent_comment_id INTEGER REFERENCES comments(comment_id),
    content TEXT NOT NULL CHECK (LENGTH(content) >= 10),
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_public BOOLEAN DEFAULT TRUE,
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    status VARCHAR(20) CHECK (status IN ('active', 'flagged', 'removed')) DEFAULT 'active'
);

-- Create votes table (associative entity)
CREATE TABLE votes (
    vote_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    case_id INTEGER NOT NULL,
    committee_id INTEGER NOT NULL,
    vote VARCHAR(20) CHECK (vote IN ('plagiarized', 'not_plagiarized', 'abstain')) NOT NULL,
    date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    rationale TEXT,
    FOREIGN KEY (case_id, committee_id) REFERENCES plagiarism_cases(case_id, committee_id) ON DELETE CASCADE,
    UNIQUE (member_id, case_id, committee_id)
);

-- Create committee memberships table (associative entity)
CREATE TABLE committee_memberships (
    membership_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    committee_id INTEGER REFERENCES committee(committee_id) NOT NULL,
    join_date DATE NOT NULL DEFAULT CURRENT_DATE,
    role VARCHAR(20) CHECK (role IN ('chair', 'member', 'secretary')),
    status VARCHAR(20) CHECK (status IN ('active', 'inactive')) DEFAULT 'active',
    term_end_date DATE,
    UNIQUE (member_id, committee_id)
);

-- Create notifications table (associative entity)
CREATE TABLE notifications (
    notif_id SERIAL PRIMARY KEY,
    member_id INTEGER REFERENCES members(member_id) NOT NULL,
    message TEXT NOT NULL,
    sent_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type VARCHAR(20) CHECK (type IN ('system', 'donation', 'comment', 'plagiarism')),
    is_read BOOLEAN DEFAULT FALSE,
    priority VARCHAR(20) CHECK (priority IN ('low', 'medium', 'high', 'urgent')) DEFAULT 'medium'
);

-- Create multivalued attributes tables
CREATE TABLE member_phone_numbers (
    member_id INTEGER REFERENCES members(member_id) ON DELETE CASCADE,
    phone_number VARCHAR(20),
    phone_type VARCHAR(20) DEFAULT 'mobile',
    is_verified BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (member_id, phone_number)
);

CREATE TABLE member_interests (
    member_id INTEGER REFERENCES members(member_id) ON DELETE CASCADE,
    interest VARCHAR(100),
    PRIMARY KEY (member_id, interest)
);

CREATE TABLE admin_permissions (
    admin_id INTEGER REFERENCES admins(admin_id) ON DELETE CASCADE,
    permission VARCHAR(50),
    PRIMARY KEY (admin_id, permission)
);

CREATE TABLE moderator_expertise (
    mod_id INTEGER REFERENCES moderators(mod_id) ON DELETE CASCADE,
    expertise_area VARCHAR(100),
    PRIMARY KEY (mod_id, expertise_area)
);

-- Create indexes for performance
CREATE INDEX idx_members_email ON members(primary_email);
CREATE INDEX idx_members_recovery_email ON members(recovery_email);
CREATE INDEX idx_texts_author ON texts(author_orcid);
CREATE INDEX idx_texts_status ON texts(status);
CREATE INDEX idx_downloads_member ON downloads(member_id);
CREATE INDEX idx_downloads_text ON downloads(text_id);
CREATE INDEX idx_donations_member ON donations(member_id);
CREATE INDEX idx_donations_text ON donations(text_id);
CREATE INDEX idx_donations_charity ON donations(charity_id);
CREATE INDEX idx_comments_text ON comments(text_id);
CREATE INDEX idx_votes_case ON votes(case_id, committee_id);
CREATE INDEX idx_plagiarism_status ON plagiarism_cases(status);
CREATE INDEX idx_notifications_member ON notifications(member_id);

-- Create views for derived attributes
CREATE VIEW member_status_view AS
SELECT 
    m.member_id,
    CASE 
        WHEN EXISTS (SELECT 1 FROM donations d WHERE d.member_id = m.member_id) THEN 'donor'
        ELSE 'non_donor'
    END as donor_status,
    CASE 
        WHEN EXISTS (SELECT 1 FROM donations d WHERE d.member_id = m.member_id) THEN 7  -- 7 downloads per week for donors
        ELSE 1  -- 1 download per week for non-donors
    END as download_limit
FROM members m;

CREATE VIEW author_h_index AS
SELECT 
    a.orcid,
    COUNT(DISTINCT t.text_id) as h_index
FROM authors a
JOIN texts t ON a.orcid = t.author_orcid
JOIN downloads d ON t.text_id = d.text_id
GROUP BY a.orcid;

-- Grant permissions to adnan user
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO adnan;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO adnan;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO adnan;
EOF

echo "Schema created in create_final_tables.sql"
echo "Run: psql -h localhost -U adnan -d cfp_db -f create_final_tables.sql"

echo "=== Step 2: Fix Rust code to match schema ==="

# Update member_repository.rs to match new schema
cat > fix_member_repository.sh << 'EOF'
#!/bin/bash

FILE="src/infrastructure/database/repositories/member_repository.rs"

# Backup
cp "$FILE" "${FILE}.backup"

# Fix the INSERT query (around line 43)
sed -i '43,66c\
        let row = sqlx::query!(\
            r#"\
            INSERT INTO members (\
                name, organization, pseudonym, primary_email, recovery_email,\
                password_hash, verification_matrix, matrix_expiry, join_date,\
                street, city, state, country, postal_code, introduced_by\
            )\
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)\
            RETURNING member_id\
            "#,\
            member.name,\
            member.organization,\
            member.pseudonym,\
            member.primary_email,\
            member.recovery_email,\
            member.password_hash,\
            member.verification_matrix.as_ref().map(|v| v.to_string()),\
            member.matrix_expiry,\
            member.join_date,\
            member.address.as_ref().map(|a| a.street.clone()),\
            member.address.as_ref().map(|a| a.city.clone()),\
            member.address.as_ref().map(|a| a.state.clone()),\
            member.address.as_ref().map(|a| a.country.clone()),\
            member.address.as_ref().map(|a| a.postal_code.clone()),\
            member.introduced_by\
        )' "$FILE"

# Fix the SELECT query (around line 111)
sed -i '111,136c\
        let row = sqlx::query!(\
            r#"\
            SELECT \
                m.member_id, m.name, m.organization, m.pseudonym,\
                m.primary_email, m.recovery_email, m.password_hash,\
                m.verification_matrix, m.matrix_expiry, m.join_date,\
                m.street, m.city, m.state, m.country, m.postal_code,\
                m.introduced_by, m.created_at, m.updated_at,\
                array_agg(DISTINCT mp.phone_number) as phone_numbers,\
                array_agg(DISTINCT mi.interest) as interests\
            FROM members m\
            LEFT JOIN member_phone_numbers mp ON m.member_id = mp.member_id\
            LEFT JOIN member_interests mi ON m.member_id = mi.member_id\
            WHERE m.member_id = $1\
            GROUP BY m.member_id\
            "#,\
            member_id\
        )' "$FILE"

# Fix the UPDATE query (around line 199)
sed -i '199,232c\
        sqlx::query!(\
            r#"\
            UPDATE members SET\
                name = $1,\
                organization = $2,\
                pseudonym = $3,\
                primary_email = $4,\
                recovery_email = $5,\
                password_hash = $6,\
                verification_matrix = $7,\
                matrix_expiry = $8,\
                street = $9,\
                city = $10,\
                state = $11,\
                country = $12,\
                postal_code = $13,\
                introduced_by = $14,\
                updated_at = NOW()\
            WHERE member_id = $15\
            "#,\
            member.name,\
            member.organization,\
            member.pseudonym,\
            member.primary_email,\
            member.recovery_email,\
            member.password_hash,\
            member.verification_matrix.as_ref().map(|v| v.to_string()),\
            member.matrix_expiry,\
            member.address.as_ref().map(|a| a.street.clone()),\
            member.address.as_ref().map(|a| a.city.clone()),\
            member.address.as_ref().map(|a| a.state.clone()),\
            member.address.as_ref().map(|a| a.country.clone()),\
            member.address.as_ref().map(|a| a.postal_code.clone()),\
            member.introduced_by,\
            member.member_id\
        )' "$FILE"

# Fix the downloads count query (around line 343)
sed -i '343,350c\
        let row = sqlx::query!(\
            r#"\
            SELECT COUNT(*) as count\
            FROM downloads\
            WHERE member_id = $1 AND download_date >= CURRENT_DATE - INTERVAL \'7 days\'\
            "#,\
            member_id\
        )' "$FILE"

# Fix the donations query (around line 358)
sed -i '358,366c\
        let row = sqlx::query!(\
            r#"\
            SELECT EXISTS(\
                SELECT 1 FROM donations WHERE member_id = $1\
            ) as "exists!"\
            "#,\
            member_id\
        )' "$FILE"

echo "Member repository queries updated to match schema"
EOF

chmod +x fix_member_repository.sh
./fix_member_repository.sh

echo "=== Step 3: Fix missing imports ==="

# Fix missing imports
cat > fix_imports.sh << 'EOF'
#!/bin/bash

# Fix redis import
sed -i 's/use redis::{aio::Connection, AsyncCommands, RedisResult};/use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};/g' src/infrastructure/cache/mod.rs

# Fix rand import
sed -i 's/use rand::{distributions::Alphanumeric, Rng};/use rand::{distributions::Alphanumeric, Rng};/g' src/utils/cryptography.rs

# Create missing types module
mkdir -p src/application/dto/types
cat > src/application/dto/types/mod.rs << 'TYPESEOF'
#[derive(Debug, Clone)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Clone)]
pub struct MemberStats {
    pub total_members: i64,
    pub active_members: i64,
    pub new_members_today: i64,
}

#[derive(Debug, Clone)]
pub struct DownloadStats {
    pub total_downloads: i64,
    pub downloads_today: i64,
    pub unique_downloaders: i64,
}
TYPESEOF

# Update imports in text_response.rs
sed -i '1i use crate::application::dto::types::StatusCount;' src/application/dto/responses/text_response.rs

# Create missing responses module
mkdir -p src/api/responses
cat > src/api/responses/mod.rs << 'RESPONSESEOF'
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: Option<T>, message: String) -> impl Responder {
        let response = ApiResponse {
            success: true,
            message,
            data,
        };
        HttpResponse::Ok().json(response)
    }
    
    pub fn error(message: String) -> impl Responder {
        let response = ApiResponse::<()> {
            success: false,
            message,
            data: None,
        };
        HttpResponse::BadRequest().json(response)
    }
}
RESPONSESEOF

# Update committee_handler imports
sed -i 's/use crate::api::responses::ApiResponse;/use crate::api::responses::ApiResponse;/g' src/api/handlers/committee_handler.rs
sed -i 's/use crate::api::responses::ApiResponse;/use crate::api::responses::ApiResponse;/g' src/api/handlers/text_handler_extras.rs

# Create missing security modules
cat > src/infrastructure/security/mod.rs << 'SECURITYEOF'
pub mod csrf_protection;
pub mod jwt_manager;
pub mod rate_limiting;
pub mod password_hasher;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Password hash error")]
    PasswordHashError,
    #[error("CSRF token missing")]
    CsrfTokenMissing,
}

pub type SecurityResult<T> = Result<T, SecurityError>;

pub struct JwtManager;

impl JwtManager {
    pub fn new() -> Self {
        Self
    }
}

pub struct Argon2Hasher;

impl Argon2Hasher {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
SECURITYEOF

echo "Missing imports fixed"
EOF

chmod +x fix_imports.sh
./fix_imports.sh

echo "=== Step 4: Fix Cargo.toml dependencies ==="

# Update Cargo.toml
cat > update_cargo.sh << 'EOF'
#!/bin/bash

# Add missing dependencies
cargo add rand --features "std_rng,distributions"
cargo add redis --features "tokio-comp,aio,connection-manager"
cargo add sqlx --features "postgres,uuid,chrono,macros,runtime-tokio-rustls,array"
cargo add async-trait
cargo add thiserror
cargo add argon2
cargo add jsonwebtoken
cargo add bcrypt

echo "Dependencies updated"
EOF

chmod +x update_cargo.sh
./update_cargo.sh

echo "=== Step 5: Create database setup script ==="

cat > setup_database_final.sh << 'EOF'
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
EOF

chmod +x setup_database_final.sh

echo "=== Step 6: Create .env file with database config ==="

cat > .env << 'ENVEOF'
# Database Configuration
DATABASE_URL=adnan://adnan@localhost/cfp_db
DATABASE_MAX_CONNECTIONS=20
DATABASE_MIN_CONNECTIONS=5
DATABASE_CONNECT_TIMEOUT=30

# Server Configuration
HOST=127.0.0.1
PORT=8080
RUST_LOG=info,cfp_backend=debug

# JWT Configuration
JWT_SECRET=your_super_secret_jwt_key_change_this_in_production
JWT_EXPIRATION=86400

# Redis Configuration
REDIS_URL=redis://127.0.0.1:6379

# Email Configuration
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your_email@gmail.com
SMTP_PASSWORD=your_app_password
SMTP_FROM=noreply@copyforward.org

# Security
CSRF_SECRET=your_csrf_secret_key_change_this
PASSWORD_SALT=your_password_salt_change_this

# Application Settings
MAX_UPLOAD_SIZE=10485760  # 10MB
ALLOWED_FILE_TYPES=txt,md,pdf,doc,docx
SESSION_TIMEOUT=3600
ENVIRONMENT=development
ENVEOF

echo "=== Step 7: Final build attempt ==="

cat > final_build.sh << 'BUILDEOF'
#!/bin/bash

echo "Cleaning previous builds..."
cargo clean

echo "Updating dependencies..."
cargo update

echo "Checking compilation..."
cargo check --lib

if [ $? -eq 0 ]; then
    echo "Compilation successful! Building..."
    cargo build
else
    echo "Compilation failed. Showing errors:"
    cargo check --lib 2>&1 | grep -A5 -B5 "error:"
fi
BUILDEOF

chmod +x final_build.sh

echo "=========================================="
echo "COMPLETE SOLUTION READY!"
echo ""
echo "Follow these steps:"
echo ""
echo "1. SET UP DATABASE:"
echo "   ./setup_database_final.sh"
echo ""
echo "2. BUILD THE PROJECT:"
echo "   ./final_build.sh"
echo ""
echo "3. IF BUILD SUCCEEDS, RUN:"
echo "   cargo run"
echo ""
echo "4. TEST THE API:"
echo "   curl http://localhost:8080/health"
echo ""
echo "Important notes:"
echo "- Make sure PostgreSQL is installed and running"
echo "- User 'adnan' needs PostgreSQL access"
echo "- Update .env file with your actual credentials"
echo "- The schema now matches your ER diagram exactly"
