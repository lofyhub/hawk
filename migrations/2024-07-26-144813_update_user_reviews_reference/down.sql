-- Revert the column name change
ALTER TABLE user_reviews
RENAME COLUMN case_id TO politician_id;

-- Drop the new foreign key constraint
ALTER TABLE user_reviews
DROP CONSTRAINT IF EXISTS user_reviews_case_id_fkey;

-- Add the old foreign key constraint
ALTER TABLE user_reviews
ADD CONSTRAINT user_reviews_politician_id_fkey FOREIGN KEY (politician_id) REFERENCES politicians(id);
