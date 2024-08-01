-- This file should undo anything in `up.sql`
-- Your SQL goes here
ALTER TABLE corruption_cases
DROP COLUMN upvotes,
DROP COLUMN downvotes;

ALTER TABLE user_reviews
DROP COLUMN upvotes,
DROP COLUMN downvotes;