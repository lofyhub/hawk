-- down.sql

-- Reverse changes in corruption_cases
ALTER TABLE corruption_cases
DROP COLUMN title;

ALTER TABLE corruption_cases
ALTER COLUMN case_description TYPE VARCHAR(255);

-- Reverse changes in user_reviews
ALTER TABLE user_reviews
RENAME COLUMN title TO case_description;

ALTER TABLE user_reviews
ALTER COLUMN review_text TYPE VARCHAR(255);
