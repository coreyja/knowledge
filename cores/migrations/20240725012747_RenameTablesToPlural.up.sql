-- This migration is NOT BLUE/GREEN SAFE
-- Any 'old' code that tries to access to table after this migration runs will FAIL
ALTER TABLE category
RENAME TO categories;

ALTER TABLE markdown
RENAME TO markdowns;

ALTER TABLE pagesnapshot
RENAME TO page_snapshots;

ALTER TABLE categorymarkdown
RENAME TO markdown_categories;
