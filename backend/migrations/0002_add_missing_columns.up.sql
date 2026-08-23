-- ============================================
-- Add all columns that the Rust code expects
-- ============================================

-- ============================================
-- ADD ALL MISSING COLUMNS FOR RUST CODE
-- ============================================

-- Add h_index to authors
ALTER TABLE authors ADD COLUMN IF NOT EXISTS h_index INTEGER DEFAULT 0;

-- Add summary columns to texts
ALTER TABLE texts 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_donations DECIMAL(10,2) DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_downloaders INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS avg_rating DECIMAL(3,2) DEFAULT 0;

-- Add summary columns to downloads
ALTER TABLE downloads 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_members INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_texts INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_downloads INTEGER DEFAULT 0;

-- Add last_7_days column (for reporting)
ALTER TABLE downloads 
ADD COLUMN IF NOT EXISTS last_7_days INTEGER DEFAULT 0;

-- Initialize data
UPDATE authors SET h_index = 0 WHERE h_index IS NULL;
UPDATE texts SET download_count = 0 WHERE download_count IS NULL;
UPDATE downloads SET download_count = 0 WHERE download_count IS NULL;



-- Downloads table - add summary columns
ALTER TABLE downloads 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_members INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_texts INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_downloads INTEGER DEFAULT 0;

-- Texts table - add missing columns
ALTER TABLE texts 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_donations INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_downloaders INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS avg_rating DECIMAL(3,2) DEFAULT 0;

-- Change abstract_text to abstract (your code expects abstract_text)
-- Actually, your database has 'abstract' but code expects 'abstract_text'
-- Let's add abstract_text as an alias
ALTER TABLE texts 
ADD COLUMN IF NOT EXISTS abstract_text TEXT;
UPDATE texts SET abstract_text = abstract WHERE abstract_text IS NULL;

-- Authors table - add missing columns
ALTER TABLE authors 
ADD COLUMN IF NOT EXISTS total_downloads INTEGER DEFAULT 0;

-- Function to update download counts
CREATE OR REPLACE FUNCTION update_download_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- Update download_count on downloads table
    UPDATE downloads 
    SET download_count = (
        SELECT COUNT(*) FROM downloads WHERE text_id = NEW.text_id
    )
    WHERE text_id = NEW.text_id;
    
    -- Update text table
    UPDATE texts 
    SET 
        download_count = (
            SELECT COUNT(*) FROM downloads WHERE text_id = NEW.text_id
        ),
        unique_downloaders = (
            SELECT COUNT(DISTINCT member_id) FROM downloads WHERE text_id = NEW.text_id
        )
    WHERE text_id = NEW.text_id;
    
    -- Update author table
    UPDATE authors 
    SET total_downloads = (
        SELECT COUNT(*) FROM downloads d 
        JOIN texts t ON d.text_id = t.text_id 
        WHERE t.author_orcid = authors.orcid
    )
    WHERE orcid = (
        SELECT author_orcid FROM texts WHERE text_id = NEW.text_id
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to keep stats updated
DROP TRIGGER IF EXISTS update_stats_on_download ON downloads;
CREATE TRIGGER update_stats_on_download
AFTER INSERT ON downloads
FOR EACH ROW
EXECUTE FUNCTION update_download_stats();

-- Initialize existing data
UPDATE downloads 
SET download_count = (
    SELECT COUNT(*) FROM downloads d2 WHERE d2.text_id = downloads.text_id
);

UPDATE texts 
SET 
    download_count = (
        SELECT COUNT(*) FROM downloads WHERE text_id = texts.text_id
    ),
    unique_downloaders = (
        SELECT COUNT(DISTINCT member_id) FROM downloads WHERE text_id = texts.text_id
    ),
    abstract_text = abstract;

UPDATE authors 
SET total_downloads = (
    SELECT COUNT(*) FROM downloads d 
    JOIN texts t ON d.text_id = t.text_id 
    WHERE t.author_orcid = authors.orcid
);
