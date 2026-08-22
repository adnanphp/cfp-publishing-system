-- backend/migrations/0002_create_functions_and_indexes.up.sql
-- Additional indexes and functions for better performance

-- Function to get plagiarism case status summary
CREATE OR REPLACE FUNCTION get_plagiarism_case_summary(p_case_id INT, p_committee_id INT)
RETURNS TABLE (
    total_votes INT,
    plagiarized_votes INT,
    not_plagiarized_votes INT,
    abstain_votes INT,
    required_majority INT,
    has_majority BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    WITH vote_counts AS (
        SELECT 
            COUNT(*) as total,
            COUNT(CASE WHEN vote = 'plagiarized' THEN 1 END) as plagiarized,
            COUNT(CASE WHEN vote = 'not_plagiarized' THEN 1 END) as not_plagiarized,
            COUNT(CASE WHEN vote = 'abstain' THEN 1 END) as abstain
        FROM votes
        WHERE case_id = p_case_id AND committee_id = p_committee_id
    )
    SELECT 
        total,
        plagiarized,
        not_plagiarized,
        abstain,
        CEIL(total * 2.0 / 3) as required_majority,
        (plagiarized >= CEIL(total * 2.0 / 3) OR not_plagiarized >= CEIL(total * 2.0 / 3)) as has_majority
    FROM vote_counts;
END;
$$ LANGUAGE plpgsql;

-- Function to check voting eligibility with time constraint
CREATE OR REPLACE FUNCTION can_vote(
    p_member_id INT,
    p_case_id INT,
    p_committee_id INT
) RETURNS TABLE (
    can_vote BOOLEAN,
    reason TEXT
) AS $$
DECLARE
    v_case_status VARCHAR(20);
    v_voting_deadline TIMESTAMP;
    v_has_voted BOOLEAN;
    v_has_downloaded BOOLEAN;
BEGIN
    -- Get case status and voting deadline
    SELECT status, opened_date + INTERVAL '14 days'
    INTO v_case_status, v_voting_deadline
    FROM plagiarism_cases
    WHERE case_id = p_case_id AND committee_id = p_committee_id;
    
    -- Check if member has already voted
    SELECT EXISTS (
        SELECT 1 FROM votes
        WHERE member_id = p_member_id 
        AND case_id = p_case_id 
        AND committee_id = p_committee_id
    ) INTO v_has_voted;
    
    -- Check if member downloaded the text
    SELECT can_vote_on_plagiarism(p_member_id, p_case_id, p_committee_id)
    INTO v_has_downloaded;
    
    -- Determine eligibility
    IF v_case_status != 'voting' THEN
        RETURN QUERY SELECT false, 'Voting is not currently open for this case';
    ELSIF v_voting_deadline < NOW() THEN
        RETURN QUERY SELECT false, 'Voting period has ended';
    ELSIF v_has_voted THEN
        RETURN QUERY SELECT false, 'You have already voted on this case';
    ELSIF NOT v_has_downloaded THEN
        RETURN QUERY SELECT false, 'You must download the text before voting';
    ELSE
        RETURN QUERY SELECT true, 'You can vote on this case';
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Function to process donation distribution
CREATE OR REPLACE FUNCTION process_donation_distribution(p_donation_id INT)
RETURNS VOID AS $$
DECLARE
    v_donation_record donations%ROWTYPE;
    v_charity_balance DECIMAL(10, 2);
    v_author_balance DECIMAL(10, 2);
    v_cfp_balance DECIMAL(10, 2);
BEGIN
    -- Get donation details
    SELECT * INTO v_donation_record
    FROM donations
    WHERE donation_id = p_donation_id;
    
    -- Update charity total received (derived attribute)
    UPDATE charities
    SET total_received = COALESCE(total_received, 0) + v_donation_record.charity_amount
    WHERE charity_id = v_donation_record.charity_id;
    
    -- Update author's total donations (derived attribute - would need author_balances table)
    -- This is simplified; in reality you'd have a separate table
    
    -- Log the distribution
    INSERT INTO donation_distributions (
        donation_id,
        charity_amount,
        author_amount,
        cfp_amount,
        distributed_at
    ) VALUES (
        p_donation_id,
        v_donation_record.charity_amount,
        v_donation_record.author_amount,
        v_donation_record.cfp_amount,
        NOW()
    );
    
    -- Update donation status
    UPDATE donations
    SET status = 'completed',
        updated_at = NOW()
    WHERE donation_id = p_donation_id;
    
    -- Create notifications
    INSERT INTO notifications (member_id, message, type, priority)
    VALUES (
        v_donation_record.member_id,
        format('Your donation of $%s has been processed successfully', v_donation_record.amount),
        'donation',
        'medium'
    );
    
    -- Notify charity (if they have a member account)
    -- This would require charity-member mapping
END;
$$ LANGUAGE plpgsql;

-- Create donation_distributions table for audit
CREATE TABLE donation_distributions (
    distribution_id SERIAL PRIMARY KEY,
    donation_id INT NOT NULL REFERENCES donations(donation_id) ON DELETE CASCADE,
    charity_amount DECIMAL(10, 2) NOT NULL,
    author_amount DECIMAL(10, 2) NOT NULL,
    cfp_amount DECIMAL(10, 2) NOT NULL,
    distributed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    notes TEXT
);

-- Index for better query performance
CREATE INDEX idx_donation_distributions_donation ON donation_distributions(donation_id);
CREATE INDEX idx_donation_distributions_date ON donation_distributions(distributed_at);

-- Create materialized view for donation analytics
CREATE MATERIALIZED VIEW donation_analytics AS
SELECT 
    DATE_TRUNC('month', d.date) as month,
    c.charity_id,
    c.name as charity_name,
    COUNT(*) as donation_count,
    SUM(d.amount) as total_amount,
    AVG(d.amount) as avg_amount,
    SUM(d.charity_amount) as total_charity_amount,
    SUM(d.author_amount) as total_author_amount,
    SUM(d.cfp_amount) as total_cfp_amount
FROM donations d
JOIN charities c ON d.charity_id = c.charity_id
WHERE d.status = 'completed'
GROUP BY DATE_TRUNC('month', d.date), c.charity_id, c.name
WITH DATA;

-- Refresh function for materialized view
CREATE OR REPLACE FUNCTION refresh_donation_analytics()
RETURNS VOID AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY donation_analytics;
END;
$$ LANGUAGE plpgsql;

-- Create index on materialized view
CREATE UNIQUE INDEX idx_donation_analytics_month_charity ON donation_analytics(month, charity_id);

-- Function to get member activity summary
CREATE OR REPLACE FUNCTION get_member_activity(p_member_id INT)
RETURNS TABLE (
    period VARCHAR(10),
    downloads_last_30_days INT,
    donations_last_30_days INT,
    comments_last_30_days INT,
    votes_last_30_days INT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        '30_days' as period,
        COUNT(DISTINCT d1.download_id) as downloads,
        COUNT(DISTINCT d2.donation_id) as donations,
        COUNT(DISTINCT c.comment_id) as comments,
        COUNT(DISTINCT v.vote_id) as votes
    FROM members m
    LEFT JOIN downloads d1 ON m.member_id = d1.member_id 
        AND d1.download_date > NOW() - INTERVAL '30 days'
    LEFT JOIN donations d2 ON m.member_id = d2.member_id 
        AND d2.date > NOW() - INTERVAL '30 days'
        AND d2.status = 'completed'
    LEFT JOIN comments c ON m.member_id = c.member_id 
        AND c.date > NOW() - INTERVAL '30 days'
        AND c.status = 'active'
    LEFT JOIN votes v ON m.member_id = v.member_id 
        AND v.date > NOW() - INTERVAL '30 days'
    WHERE m.member_id = p_member_id
    GROUP BY m.member_id;
END;
$$ LANGUAGE plpgsql;

-- Create composite indexes for better join performance
CREATE INDEX idx_texts_composite_status_date ON texts(status, upload_date);
CREATE INDEX idx_donations_composite_status_date ON donations(status, date);
CREATE INDEX idx_comments_composite_text_status ON comments(text_id, status);
CREATE INDEX idx_votes_composite_case_committee ON votes(case_id, committee_id, date);

-- Function to update derived text statistics
CREATE OR REPLACE FUNCTION update_text_statistics(p_text_id INT)
RETURNS VOID AS $$
BEGIN
    -- This would update a separate statistics table
    -- For now, we rely on the view text_summary
    -- In production, you might want to materialize this
    NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update text statistics after download
CREATE OR REPLACE FUNCTION trigger_update_text_stats()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM update_text_statistics(NEW.text_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_text_stats_after_download
AFTER INSERT ON downloads
FOR EACH ROW EXECUTE FUNCTION trigger_update_text_stats();

CREATE TRIGGER update_text_stats_after_donation
AFTER INSERT OR UPDATE ON donations
FOR EACH ROW EXECUTE FUNCTION trigger_update_text_stats();

CREATE TRIGGER update_text_stats_after_comment
AFTER INSERT OR UPDATE ON comments
FOR EACH ROW EXECUTE FUNCTION trigger_update_text_stats();
