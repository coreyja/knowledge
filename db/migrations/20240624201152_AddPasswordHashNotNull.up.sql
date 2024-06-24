-- Add migration script here
ALTER TABLE users 
ALTER COLUMN password_hash SET NOT NULL;
