-- Your SQL goes here
ALTER TABLE corruption_cases
ADD COLUMN upvotes INT DEFAULT 0,
ADD COLUMN downvotes INT DEFAULT 0;

ALTER TABLE user_reviews
ADD COLUMN upvotes INT DEFAULT 0,
ADD COLUMN downvotes INT DEFAULT 0;