-- Add migration script here
ALTER TABLE PageSnapshot ADD COLUMN summary TEXT DEFAULT '' NOT NULL;
