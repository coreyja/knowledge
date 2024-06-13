-- Add migration script here
CREATE TABLE IF NOT EXISTS Category (
    markdown_id UUID PRIMARY KEY REFERENCES Markdown(markdown_id),
    category TEXT
);
