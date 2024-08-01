
-- Apply changes to corruption_cases
ALTER TABLE corruption_cases
ADD COLUMN title VARCHAR(255);

-- In PostgreSQL, MODIFY COLUMN is done using ALTER COLUMN
ALTER TABLE corruption_cases
ALTER COLUMN case_description TYPE VARCHAR(500);

-- Apply changes to user_reviews
ALTER TABLE user_reviews
RENAME COLUMN case_description TO title;

-- In PostgreSQL, MODIFY COLUMN is done using ALTER COLUMN
ALTER TABLE user_reviews
ALTER COLUMN review_text TYPE VARCHAR(500);
