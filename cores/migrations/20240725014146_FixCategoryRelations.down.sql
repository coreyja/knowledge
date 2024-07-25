-- Add migration script here
ALTER TABLE categories
ADD COLUMN markdown_id uuid REFERENCES markdowns (id);
