-- ============================================
-- Rollback migration
-- ============================================

DROP TRIGGER IF EXISTS update_counts_on_download ON downloads;
DROP FUNCTION IF EXISTS update_download_counts();

ALTER TABLE downloads 
DROP COLUMN IF EXISTS download_count,
DROP COLUMN IF EXISTS unique_members,
DROP COLUMN IF EXISTS unique_texts,
DROP COLUMN IF EXISTS total_downloads;

ALTER TABLE texts 
DROP COLUMN IF EXISTS download_count,
DROP COLUMN IF EXISTS total_donations,
DROP COLUMN IF EXISTS unique_downloaders;

ALTER TABLE authors 
DROP COLUMN IF EXISTS total_downloads,
DROP COLUMN IF EXISTS h_index;
ALTER TABLE authors DROP COLUMN IF EXISTS h_index;
ALTER TABLE texts DROP COLUMN IF EXISTS download_count;
ALTER TABLE texts DROP COLUMN IF EXISTS total_donations;
ALTER TABLE texts DROP COLUMN IF EXISTS unique_downloaders;
ALTER TABLE texts DROP COLUMN IF EXISTS avg_rating;
ALTER TABLE downloads DROP COLUMN IF EXISTS download_count;
ALTER TABLE downloads DROP COLUMN IF EXISTS unique_members;
ALTER TABLE downloads DROP COLUMN IF EXISTS unique_texts;
ALTER TABLE downloads DROP COLUMN IF EXISTS total_downloads;
ALTER TABLE downloads DROP COLUMN IF EXISTS last_7_days;
