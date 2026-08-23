-- ============================================
-- Migration: Add summary columns for downloads
-- ============================================

-- Add columns to downloads table for aggregated data
ALTER TABLE downloads 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_members INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_texts INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_downloads INTEGER DEFAULT 0;

-- Add columns to texts table for aggregated data
ALTER TABLE texts 
ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS total_donations DECIMAL(10,2) DEFAULT 0,
ADD COLUMN IF NOT EXISTS unique_downloaders INTEGER DEFAULT 0;

-- Add columns to authors table for aggregated data
ALTER TABLE authors 
ADD COLUMN IF NOT EXISTS total_downloads INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS h_index INTEGER DEFAULT 0;

-- Create function to update download counts
CREATE OR REPLACE FUNCTION update_download_counts()
RETURNS TRIGGER AS $$
BEGIN
    -- Update text download count
    UPDATE texts 
    SET download_count = (
        SELECT COUNT(*) FROM downloads WHERE text_id = NEW.text_id
    )
    WHERE text_id = NEW.text_id;
    
    -- Update author total downloads
    UPDATE authors 
    SET total_downloads = (
        SELECT COUNT(*) FROM downloads d 
        JOIN texts t ON d.text_id = t.text_id 
        WHERE t.author_orcid = (
            SELECT author_orcid FROM texts WHERE text_id = NEW.text_id
        )
    )
    WHERE orcid = (
        SELECT author_orcid FROM texts WHERE text_id = NEW.text_id
    );
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to update counts on new downloads
CREATE TRIGGER update_counts_on_download
AFTER INSERT ON downloads
FOR EACH ROW
EXECUTE FUNCTION update_download_counts();

-- Initialize existing data
UPDATE texts t 
SET download_count = (
    SELECT COUNT(*) FROM downloads WHERE text_id = t.text_id
);

UPDATE authors a 
SET total_downloads = (
    SELECT COUNT(*) FROM downloads d 
    JOIN texts t ON d.text_id = t.text_id 
    WHERE t.author_orcid = a.orcid
);
