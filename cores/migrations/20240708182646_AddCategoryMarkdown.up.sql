-- Drop the existing primary key constraint
ALTER TABLE Category DROP CONSTRAINT IF EXISTS category_pkey;

-- Ensure Category table has category_id column with a default UUID value
ALTER TABLE Category ADD COLUMN IF NOT EXISTS category_id UUID DEFAULT gen_random_uuid() NOT NULL;

-- Set category_id as the new primary key
ALTER TABLE Category ADD CONSTRAINT category_pkey PRIMARY KEY (category_id);

-- Add migration script here
CREATE TABLE IF NOT EXISTS CategoryMarkdown (
    category_id UUID NOT NULL REFERENCES Category(category_id),
    markdown_id UUID REFERENCES Markdown(markdown_id),
    PRIMARY KEY (category_id, markdown_id)
);
