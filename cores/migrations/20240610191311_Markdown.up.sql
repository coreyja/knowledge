-- Add migration script here
CREATE TABLE IF NOT EXISTS Markdown (
    markdown_id UUID PRIMARY KEY,
    title TEXT,
    content_md TEXT NOT NULL
);
