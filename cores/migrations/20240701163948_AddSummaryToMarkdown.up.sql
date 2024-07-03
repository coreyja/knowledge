-- Add migration script here
ALTER TABLE markdown ADD COLUMN summary TEXT NOT NULL DEFAULT '';
