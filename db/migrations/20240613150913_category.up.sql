-- Add migration script here
CREATE TABLE IF NOT EXISTS Category (
    note_id UUID PRIMARY KEY,
    category TEXT
);
