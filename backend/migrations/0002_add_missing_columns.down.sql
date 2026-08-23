-- ============================================
-- Rollback
-- ============================================

DROP TRIGGER IF EXISTS update_stats_on_download ON downloads;
DROP FUNCTION IF EXISTS update_download_stats();

ALTER TABLE downloads 
DROP COLUMN IF EXISTS download_count,
DROP COLUMN IF EXISTS unique_members,
DROP COLUMN IF EXISTS unique_texts,
DROP COLUMN IF EXISTS total_downloads;

ALTER TABLE texts 
DROP COLUMN IF EXISTS download_count,
DROP COLUMN IF EXISTS total_donations,
DROP COLUMN IF EXISTS unique_downloaders,
DROP COLUMN IF EXISTS avg_rating,
DROP COLUMN IF EXISTS abstract_text;

ALTER TABLE authors 
DROP COLUMN IF EXISTS total_downloads;
