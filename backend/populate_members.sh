#!/bin/bash

echo "Populating members table..."

psql "postgresql://adnan:shahzadma@localhost:5432/cfp_db" << 'SQL'
-- Insert sample members
INSERT INTO members (name, email, status) VALUES
('Alice Johnson', 'alice@cfp.com', 'active'),
('David Brown', 'david@cfp.com', 'active'),
('Emma Wilson', 'emma@cfp.com', 'pending'),
('Michael Chen', 'michael@cfp.com', 'active'),
('Sarah Davis', 'sarah@cfp.com', 'suspended')
ON CONFLICT (email) DO NOTHING;

-- Show results
SELECT '✅ Members added!' as message;
SELECT COUNT(*) as total_members FROM members;
SELECT id, name, email, status FROM members;
SQL

echo ""
echo "✅ Members table populated!"
echo ""
echo "Now test: curl http://localhost:3000/api/members"
