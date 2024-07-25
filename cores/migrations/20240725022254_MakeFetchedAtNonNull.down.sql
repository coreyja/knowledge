-- Add migration script here
ALTER TABLE page_snapshots
ALTER COLUMN fetched_at
DROP NOT NULL;
