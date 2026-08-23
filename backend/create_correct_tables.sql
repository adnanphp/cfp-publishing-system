-- Drop existing tables if they exist
DROP TABLE IF EXISTS member_interests CASCADE;
DROP TABLE IF EXISTS member_phone_numbers CASCADE;
DROP TABLE IF EXISTS downloads CASCADE;
DROP TABLE IF EXISTS donations CASCADE;
DROP TABLE IF EXISTS members CASCADE;

-- Create members table with correct schema
CREATE TABLE members (
    member_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    organization VARCHAR(255),
    pseudonym VARCHAR(100),
    primary_email VARCHAR(255) UNIQUE NOT NULL,
    recovery_email VARCHAR(255),
    password_hash VARCHAR(255) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    verification_token VARCHAR(100),
    verification_expires_at TIMESTAMP,
    role VARCHAR(50) DEFAULT 'member',
    status VARCHAR(50) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP,
    introduced_by UUID REFERENCES members(member_id),
    recovery_token VARCHAR(100),
    recovery_expires_at TIMESTAMP,
    street VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100),
    postal_code VARCHAR(20),
    verification_matrix TEXT
);

-- Create member_phone_numbers table
CREATE TABLE member_phone_numbers (
    phone_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    phone_number VARCHAR(50) NOT NULL,
    phone_type VARCHAR(50) DEFAULT 'mobile',
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Create member_interests table
CREATE TABLE member_interests (
    interest_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    interest VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(member_id, interest)
);

-- Create donations table
CREATE TABLE donations (
    donation_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    donor_id UUID REFERENCES members(member_id),
    text_id UUID,
    amount DECIMAL(10,2) NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD',
    payment_method VARCHAR(50),
    transaction_id VARCHAR(255) UNIQUE,
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP,
    member_id UUID REFERENCES members(member_id)
);

-- Create downloads table
CREATE TABLE downloads (
    download_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID REFERENCES members(member_id),
    text_id UUID,
    downloaded_at TIMESTAMP DEFAULT NOW(),
    ip_address INET,
    user_agent TEXT,
    download_date TIMESTAMP DEFAULT NOW()
);

-- Grant permissions (adjust for your user)
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO your_username;

-- Create indexes
CREATE INDEX idx_members_email ON members(primary_email);
CREATE INDEX idx_members_recovery_email ON members(recovery_email);
CREATE INDEX idx_member_phones_member_id ON member_phone_numbers(member_id);
CREATE INDEX idx_member_interests_member_id ON member_interests(member_id);
CREATE INDEX idx_donations_donor_id ON donations(donor_id);
CREATE INDEX idx_donations_member_id ON donations(member_id);
CREATE INDEX idx_downloads_member_id ON downloads(member_id);
