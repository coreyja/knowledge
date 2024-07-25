-- Add migration script here
ALTER TABLE page_snapshots
ALTER COLUMN fetched_at
SET
  NOT NULL;
