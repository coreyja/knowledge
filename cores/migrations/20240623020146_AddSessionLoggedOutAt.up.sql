-- Add migration script here
ALTER TABLE Sessions
ADD COLUMN logged_out_at TIMESTAMPTZ NULL;
