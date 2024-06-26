-- Add migration script here
ALTER TABLE Users
ADD COLUMN password_hash TEXT NULL;
