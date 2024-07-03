-- Add migration script here
ALTER TABLE PageSnapshot ADD COLUMN cleaned_html TEXT NOT NULL;
