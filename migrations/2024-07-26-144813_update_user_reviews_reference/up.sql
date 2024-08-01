-- Rename the column
ALTER TABLE user_reviews
RENAME COLUMN politician_id TO case_id;

-- Drop the old foreign key constraint if it exists
ALTER TABLE user_reviews
DROP CONSTRAINT IF EXISTS user_reviews_politician_id_fkey;

-- Add the new foreign key constraint
ALTER TABLE user_reviews
ADD CONSTRAINT user_reviews_case_id_fkey FOREIGN KEY (case_id) REFERENCES corruption_cases(id);
