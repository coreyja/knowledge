ALTER TABLE categories ADD CONSTRAINT category_id_unique UNIQUE (category_id);

ALTER TABLE categories
DROP CONSTRAINT categories_pkey;
