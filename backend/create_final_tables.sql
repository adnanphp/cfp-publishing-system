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
