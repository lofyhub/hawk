-- This file should undo anything in `up.sql`
-- 2024-07-25-123459_fix_column_name_typo/down.sql
ALTER TABLE user_reviews
RENAME COLUMN review_text TO revieww_text;