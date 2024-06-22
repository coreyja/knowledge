-- Add migration script here
ALTER TABLE Users
DROP COLUMN password_hash;
