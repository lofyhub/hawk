-- This file should undo anything in `up.sql`
ALTER TABLE corruption_cases 
ALTER COLUMN case_description TYPE VARCHAR(1000)