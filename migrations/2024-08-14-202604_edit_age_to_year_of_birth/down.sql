-- This file should undo anything in `up.sql`
ALTER TABLE politicians
RENAME COLUMN year_of_birth TO age;

