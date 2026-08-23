-- Add test members to your existing table
-- Note: We need to provide required columns: member_id, name, primary_email, password_hash, join_date
INSERT INTO members (member_id, name, primary_email, password_hash, join_date) VALUES
(1, 'John Doe', 'john@cfp.com', 'hashed_password_123', CURRENT_DATE),
(2, 'Jane Smith', 'jane@cfp.com', 'hashed_password_456', CURRENT_DATE),
(3, 'Bob Wilson', 'bob@cfp.com', 'hashed_password_789', CURRENT_DATE)
ON CONFLICT (member_id) DO NOTHING;

-- Verify
SELECT '✅ Test members added!' as message;
SELECT COUNT(*) as total_members FROM members;
SELECT member_id, name, primary_email, join_date FROM members;
