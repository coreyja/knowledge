ALTER TABLE markdowns
ALTER COLUMN summary
SET DEFAULT '';

ALTER TABLE markdowns
ALTER COLUMN summary
SET
  NOT NULL;
