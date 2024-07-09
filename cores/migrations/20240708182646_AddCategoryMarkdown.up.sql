-- Add category_id column ONLY if it doesn't exist
ALTER TABLE Category ADD COLUMN IF NOT EXISTS category_id UUID DEFAULT gen_random_uuid();

-- Populates it with unique UUIDs for existing rows.
UPDATE Category SET category_id = gen_random_uuid() WHERE category_id IS NULL;

-- Makes it NOT NULL after ensuring all rows have a value.
ALTER TABLE Category ALTER COLUMN category_id SET NOT NULL;

-- Add a unique constraint on category_id
ALTER TABLE Category ADD CONSTRAINT category_id_unique UNIQUE (category_id);


CREATE TABLE IF NOT EXISTS CategoryMarkdown (
    category_id UUID NOT NULL REFERENCES Category(category_id),
    markdown_id UUID REFERENCES Markdown(markdown_id),
    PRIMARY KEY (category_id, markdown_id)
);
