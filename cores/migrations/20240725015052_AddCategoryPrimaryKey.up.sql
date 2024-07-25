-- Add Primary Key to Categories on category_id
ALTER TABLE categories ADD PRIMARY KEY (category_id);

ALTER TABLE categories
DROP CONSTRAINT category_id_unique CASCADE;
