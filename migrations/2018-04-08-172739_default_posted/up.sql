ALTER TABLE threads ALTER COLUMN posted SET DEFAULT (NOW() AT TIME ZONE 'UTC');
ALTER TABLE posts ALTER COLUMN posted SET DEFAULT (NOW() AT TIME ZONE 'UTC');
