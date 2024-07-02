-- Add migration script here
ALTER TABLE Sessions
DROP COLUMN logged_out_at;
