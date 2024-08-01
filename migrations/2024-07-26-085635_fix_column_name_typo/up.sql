-- 2024-07-25-123459_fix_column_name_typo/up.sql
ALTER TABLE user_reviews
RENAME COLUMN revieww_text TO review_text;