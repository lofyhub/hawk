-- This file should undo anything in `up.sql`
ALTER TABLE politicians
DROP COLUMN age;

ALTER TABLE politicians
DROP COLUMN sex;