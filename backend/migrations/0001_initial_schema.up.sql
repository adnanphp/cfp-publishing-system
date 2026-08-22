-- backend/migrations/0001_initial_schema.up.sql
-- ============================================
-- CFP Publishing System - Initial Schema
-- Based on ER Diagram with all concepts
-- ============================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable citext for case-insensitive email
CREATE EXTENSION IF NOT EXISTS "citext";

-- Enable pgcrypto for password hashing
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Enable unaccent for better search
CREATE EXTENSION IF NOT EXISTS "unaccent";

-- ===== STRONG ENTITIES =====

-- Member table (Strong Entity)
CREATE TABLE members (
    member_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    organization VARCHAR(100),
    pseudonym VARCHAR(100),
    
    -- Contact information
    primary_email CITEXT UNIQUE NOT NULL,
    recovery_email CITEXT,
    password_hash VARCHAR(255) NOT NULL,
    
    -- Verification
    verification_matrix VARCHAR(255),
    matrix_expiry TIMESTAMP WITH TIME ZONE,
    
    -- Address (Composite Attribute)
    street VARCHAR(100),
    city VARCHAR(50),
    state VARCHAR(50),
    country VARCHAR(50),
    postal_code VARCHAR(20),
    
    -- Dates
    join_date DATE NOT NULL DEFAULT CURRENT_DATE,
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Recursive relationship
    introduced_by INT REFERENCES members(member_id),
    
    -- Constraints
    CONSTRAINT valid_email CHECK (primary_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT valid_recovery_email CHECK (recovery_email IS NULL OR recovery_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT matrix_expiry_check CHECK (matrix_expiry > created_at)
);

-- Phone numbers (Multivalued Attribute)
CREATE TABLE member_phone_numbers (
    phone_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    phone_number VARCHAR(20) NOT NULL,
    phone_type VARCHAR(20) DEFAULT 'mobile',
    is_verified BOOLEAN DEFAULT false,
    UNIQUE(member_id, phone_number)
);

-- Areas of interest (Multivalued Attribute)
CREATE TABLE member_interests (
    interest_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    interest VARCHAR(100) NOT NULL,
    UNIQUE(member_id, interest)
);

-- Author specialization (from generalization)
CREATE TABLE authors (
    orcid VARCHAR(20) PRIMARY KEY,
    member_id INT UNIQUE NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    bio TEXT,
    specialization VARCHAR(100),
    website_url VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Admin specialization
CREATE TABLE admins (
    admin_id SERIAL PRIMARY KEY,
    member_id INT UNIQUE NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('super', 'content', 'financial')),
    permissions JSONB NOT NULL DEFAULT '[]',
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Moderator specialization
CREATE TABLE moderators (
    mod_id SERIAL PRIMARY KEY,
    member_id INT UNIQUE NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    domain VARCHAR(50),
    expertise_area JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Message entity
CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    sender_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    recipient_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    subject VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_read BOOLEAN NOT NULL DEFAULT false,
    is_deleted_sender BOOLEAN NOT NULL DEFAULT false,
    is_deleted_recipient BOOLEAN NOT NULL DEFAULT false,
    
    CONSTRAINT different_sender_recipient CHECK (sender_id != recipient_id)
);

-- Text entity
CREATE TABLE texts (
    text_id SERIAL PRIMARY KEY,
    author_orcid VARCHAR(20) NOT NULL REFERENCES authors(orcid) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    abstract TEXT,
    topic VARCHAR(100),
    version INT NOT NULL DEFAULT 1,
    upload_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('draft', 'under_review', 'published', 'archived')) DEFAULT 'draft',
    file_path VARCHAR(500),
    file_size BIGINT,
    file_hash VARCHAR(64),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    UNIQUE(author_orcid, title, version)
);

-- Keywords (Multivalued Attribute for Text)
CREATE TABLE text_keywords (
    keyword_id SERIAL PRIMARY KEY,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    keyword VARCHAR(100) NOT NULL,
    UNIQUE(text_id, keyword)
);

-- Charity entity
CREATE TABLE charities (
    charity_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    mission VARCHAR(255),
    country VARCHAR(50),
    registration_number VARCHAR(50) UNIQUE NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'inactive', 'pending')) DEFAULT 'pending',
    website_url VARCHAR(255),
    contact_email CITEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Committee entity
CREATE TABLE committees (
    committee_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    purpose TEXT,
    scope VARCHAR(20) NOT NULL CHECK (scope IN ('plagiarism', 'content', 'finance', 'appeals')),
    formation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'inactive')) DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- ===== WEAK ENTITIES =====

-- TextVersion (Weak Entity)
CREATE TABLE text_versions (
    version_id SERIAL,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    changes TEXT NOT NULL,
    submitted_date DATE NOT NULL DEFAULT CURRENT_DATE,
    review_date DATE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'approved', 'rejected')) DEFAULT 'pending',
    change_summary VARCHAR(255),
    moderator_id INT REFERENCES admins(admin_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (version_id, text_id)
);

-- PlagiarismCase (Weak Entity)
CREATE TABLE plagiarism_cases (
    case_id SERIAL,
    committee_id INT NOT NULL REFERENCES committees(committee_id) ON DELETE CASCADE,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    opened_date DATE NOT NULL DEFAULT CURRENT_DATE,
    description TEXT,
    status VARCHAR(20) NOT NULL CHECK (status IN ('open', 'under_review', 'voting', 'closed', 'appealed')) DEFAULT 'open',
    resolution VARCHAR(20) CHECK (resolution IN ('plagiarized', 'not_plagiarized', 'appealed')),
    closed_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (case_id, committee_id)
);

-- ===== ASSOCIATIVE ENTITIES =====

-- Download (Associative Entity)
CREATE TABLE downloads (
    download_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    download_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    ip_address INET,
    user_agent VARCHAR(255),
    country VARCHAR(50),
    
    UNIQUE(member_id, text_id, download_date) -- Prevent duplicate downloads in same second
);

-- Donation (Associative Entity with complex constraints)
CREATE TABLE donations (
    donation_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    charity_id INT NOT NULL REFERENCES charities(charity_id) ON DELETE CASCADE,
    amount DECIMAL(10, 2) NOT NULL CHECK (amount >= 1.00),
    date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    currency CHAR(3) NOT NULL DEFAULT 'USD',
    payment_method VARCHAR(50) NOT NULL,
    transaction_id VARCHAR(100) UNIQUE NOT NULL,
    
    -- Percentage distribution
    charity_pct INT NOT NULL CHECK (charity_pct >= 60 AND charity_pct <= 100),
    cfp_pct INT NOT NULL CHECK (cfp_pct >= 0),
    author_pct INT NOT NULL CHECK (author_pct >= 0),
    
    -- Derived amounts (calculated on insert/update)
    charity_amount DECIMAL(10, 2) GENERATED ALWAYS AS (amount * charity_pct / 100) STORED,
    cfp_amount DECIMAL(10, 2) GENERATED ALWAYS AS (amount * cfp_pct / 100) STORED,
    author_amount DECIMAL(10, 2) GENERATED ALWAYS AS (amount * author_pct / 100) STORED,
    
    -- Status
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'refunded')) DEFAULT 'pending',
    failure_reason TEXT,
    
    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_percentages CHECK (charity_pct + cfp_pct + author_pct = 100)
);

-- Comment (Associative Entity with hierarchical structure)
CREATE TABLE comments (
    comment_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    text_id INT NOT NULL REFERENCES texts(text_id) ON DELETE CASCADE,
    parent_comment_id INT REFERENCES comments(comment_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_public BOOLEAN NOT NULL DEFAULT true,
    rating INT CHECK (rating >= 1 AND rating <= 5),
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'flagged', 'removed')) DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT content_length CHECK (length(content) >= 10),
    CONSTRAINT max_depth CHECK (
        parent_comment_id IS NULL OR 
        NOT EXISTS (
            SELECT 1 FROM comments c2 
            WHERE c2.comment_id = comments.parent_comment_id 
            AND c2.parent_comment_id IS NOT NULL
        )
    )
);

-- Vote (Associative Entity for plagiarism cases)
CREATE TABLE votes (
    vote_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    case_id INT NOT NULL,
    committee_id INT NOT NULL,
    vote VARCHAR(20) NOT NULL CHECK (vote IN ('plagiarized', 'not_plagiarized', 'abstain')),
    date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    rationale TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (case_id, committee_id) REFERENCES plagiarism_cases(case_id, committee_id) ON DELETE CASCADE,
    UNIQUE(member_id, case_id, committee_id)
);

-- CommitteeMembership (Associative Entity)
CREATE TABLE committee_memberships (
    membership_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    committee_id INT NOT NULL REFERENCES committees(committee_id) ON DELETE CASCADE,
    join_date DATE NOT NULL DEFAULT CURRENT_DATE,
    role VARCHAR(20) NOT NULL CHECK (role IN ('chair', 'member', 'secretary')) DEFAULT 'member',
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'inactive')) DEFAULT 'active',
    term_end_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    UNIQUE(member_id, committee_id)
);

-- Notification (Associative Entity)
CREATE TABLE notifications (
    notif_id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(member_id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    sent_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    type VARCHAR(20) NOT NULL CHECK (type IN ('system', 'donation', 'comment', 'plagiarism', 'message')),
    priority VARCHAR(10) NOT NULL CHECK (priority IN ('low', 'medium', 'high', 'urgent')) DEFAULT 'medium',
    is_read BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- ===== INDEXES FOR PERFORMANCE =====

-- Member indexes
CREATE INDEX idx_members_email ON members(primary_email);
CREATE INDEX idx_members_join_date ON members(join_date);
CREATE INDEX idx_members_introduced_by ON members(introduced_by);

-- Author indexes
CREATE INDEX idx_authors_member_id ON authors(member_id);

-- Text indexes
CREATE INDEX idx_texts_author ON texts(author_orcid);
CREATE INDEX idx_texts_status ON texts(status);
CREATE INDEX idx_texts_upload_date ON texts(upload_date);
CREATE INDEX idx_texts_topic ON texts(topic);

-- Download indexes
CREATE INDEX idx_downloads_member ON downloads(member_id);
CREATE INDEX idx_downloads_text ON downloads(text_id);
CREATE INDEX idx_downloads_date ON downloads(download_date);

-- Donation indexes
CREATE INDEX idx_donations_member ON donations(member_id);
CREATE INDEX idx_donations_text ON donations(text_id);
CREATE INDEX idx_donations_charity ON donations(charity_id);
CREATE INDEX idx_donations_date ON donations(date);
CREATE INDEX idx_donations_status ON donations(status);

-- Comment indexes
CREATE INDEX idx_comments_text ON comments(text_id);
CREATE INDEX idx_comments_member ON comments(member_id);
CREATE INDEX idx_comments_parent ON comments(parent_comment_id);
CREATE INDEX idx_comments_date ON comments(date);

-- Vote indexes
CREATE INDEX idx_votes_case ON votes(case_id, committee_id);
CREATE INDEX idx_votes_member ON votes(member_id);

-- Message indexes
CREATE INDEX idx_messages_sender ON messages(sender_id);
CREATE INDEX idx_messages_recipient ON messages(recipient_id);
CREATE INDEX idx_messages_sent_at ON messages(sent_at);

-- Notification indexes
CREATE INDEX idx_notifications_member ON notifications(member_id);
CREATE INDEX idx_notifications_read ON notifications(is_read);
CREATE INDEX idx_notifications_sent_date ON notifications(sent_date);

-- ===== FULL-TEXT SEARCH INDEXES =====

-- For text search
ALTER TABLE texts ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(abstract, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(topic, '')), 'C')
    ) STORED;

CREATE INDEX idx_texts_search ON texts USING gin(search_vector);

-- For member search
ALTER TABLE members ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (
        setweight(to_tsvector('english', coalesce(name, '')), 'A') ||
        setweight(to_tsvector('english', coalesce(organization, '')), 'B') ||
        setweight(to_tsvector('english', coalesce(pseudonym, '')), 'C')
    ) STORED;

CREATE INDEX idx_members_search ON members USING gin(search_vector);

-- ===== FUNCTIONS AND TRIGGERS =====

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for updated_at
CREATE TRIGGER update_members_updated_at BEFORE UPDATE ON members
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_texts_updated_at BEFORE UPDATE ON texts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_charities_updated_at BEFORE UPDATE ON charities
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_donations_updated_at BEFORE UPDATE ON donations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to check if member can vote (must have downloaded the text)
CREATE OR REPLACE FUNCTION can_vote_on_plagiarism(
    p_member_id INT,
    p_case_id INT,
    p_committee_id INT
) RETURNS BOOLEAN AS $$
DECLARE
    v_text_id INT;
    v_has_downloaded BOOLEAN;
BEGIN
    -- Get text_id from the case
    SELECT text_id INTO v_text_id
    FROM plagiarism_cases
    WHERE case_id = p_case_id AND committee_id = p_committee_id;
    
    -- Check if member downloaded this text
    SELECT EXISTS (
        SELECT 1 FROM downloads
        WHERE member_id = p_member_id AND text_id = v_text_id
    ) INTO v_has_downloaded;
    
    RETURN v_has_downloaded;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate member status (derived attribute)
CREATE OR REPLACE FUNCTION calculate_member_status(p_member_id INT)
RETURNS VARCHAR(20) AS $$
DECLARE
    v_is_donor BOOLEAN;
    v_download_count INT;
    v_status VARCHAR(20);
BEGIN
    -- Check if member has made donations
    SELECT EXISTS (
        SELECT 1 FROM donations 
        WHERE member_id = p_member_id AND status = 'completed'
    ) INTO v_is_donor;
    
    -- Count downloads in last 30 days
    SELECT COUNT(*) INTO v_download_count
    FROM downloads
    WHERE member_id = p_member_id 
    AND download_date > NOW() - INTERVAL '30 days';
    
    -- Determine status based on business rules
    IF v_is_donor THEN
        IF v_download_count >= 30 THEN
            v_status := 'premium';
        ELSE
            v_status := 'donor';
        END IF;
    ELSE
        IF v_download_count >= 10 THEN
            v_status := 'active';
        ELSE
            v_status := 'basic';
        END IF;
    END IF;
    
    RETURN v_status;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate download limit (derived attribute)
CREATE OR REPLACE FUNCTION calculate_download_limit(p_member_id INT)
RETURNS INT AS $$
DECLARE
    v_is_donor BOOLEAN;
BEGIN
    -- Check if member has made donations
    SELECT EXISTS (
        SELECT 1 FROM donations 
        WHERE member_id = p_member_id AND status = 'completed'
    ) INTO v_is_donor;
    
    -- Business rule: donors get 1/day, non-donors get 1/week
    IF v_is_donor THEN
        RETURN 1; -- per day
    ELSE
        RETURN 1; -- per week (handled by application logic)
    END IF;
END;
$$ LANGUAGE plpgsql;

-- View for member summary (including derived attributes)
CREATE VIEW member_summary AS
SELECT 
    m.member_id,
    m.name,
    m.primary_email,
    m.join_date,
    calculate_member_status(m.member_id) as status,
    calculate_download_limit(m.member_id) as download_limit,
    COUNT(DISTINCT d.donation_id) as total_donations,
    COUNT(DISTINCT dl.download_id) as total_downloads,
    COUNT(DISTINCT c.comment_id) as total_comments,
    MAX(dl.download_date) as last_download
FROM members m
LEFT JOIN donations d ON m.member_id = d.member_id AND d.status = 'completed'
LEFT JOIN downloads dl ON m.member_id = dl.member_id
LEFT JOIN comments c ON m.member_id = c.member_id AND c.status = 'active'
GROUP BY m.member_id;

-- View for text summary (including derived attributes)
CREATE VIEW text_summary AS
SELECT 
    t.text_id,
    t.title,
    t.author_orcid,
    t.status,
    t.upload_date,
    COUNT(DISTINCT dl.download_id) as download_count,
    COALESCE(SUM(d.amount), 0) as total_donations,
    COALESCE(AVG(c.rating), 0) as avg_rating,
    COUNT(DISTINCT c.comment_id) as comment_count
FROM texts t
LEFT JOIN downloads dl ON t.text_id = dl.text_id
LEFT JOIN donations d ON t.text_id = d.text_id AND d.status = 'completed'
LEFT JOIN comments c ON t.text_id = c.text_id AND c.status = 'active'
GROUP BY t.text_id;

-- Function to validate donation percentages
CREATE OR REPLACE FUNCTION validate_donation_percentages()
RETURNS TRIGGER AS $$
BEGIN
    -- Business rule: charity must get at least 60%
    IF NEW.charity_pct < 60 THEN
        RAISE EXCEPTION 'Charity percentage must be at least 60%%';
    END IF;
    
    -- Total must be 100%
    IF NEW.charity_pct + NEW.cfp_pct + NEW.author_pct != 100 THEN
        RAISE EXCEPTION 'Percentages must sum to 100%%';
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER validate_donation_percentages_trigger 
BEFORE INSERT OR UPDATE ON donations
FOR EACH ROW EXECUTE FUNCTION validate_donation_percentages();

-- ===== SAMPLE DATA =====

-- Insert sample charities
INSERT INTO charities (name, description, mission, country, registration_number, status) VALUES
('Education for All', 'Providing education to underprivileged children worldwide', 'To ensure every child has access to quality education', 'Global', 'Edu-12345', 'active'),
('Open Access Research', 'Supporting open access academic publishing', 'Making research freely available to everyone', 'Global', 'OAR-67890', 'active'),
('Climate Action Fund', 'Funding research on climate change solutions', 'Accelerating climate change mitigation research', 'Global', 'CAF-54321', 'active');

-- Insert sample committee
INSERT INTO committees (name, purpose, scope, formation_date) VALUES
('Plagiarism Review Committee', 'Review and decide on plagiarism cases', 'plagiarism', CURRENT_DATE),
('Content Quality Committee', 'Ensure high quality of published texts', 'content', CURRENT_DATE),
('Financial Oversight Committee', 'Monitor donations and distributions', 'finance', CURRENT_DATE);
