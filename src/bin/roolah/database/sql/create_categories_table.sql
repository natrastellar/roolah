CREATE TABLE IF NOT EXISTS categories (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != '')
)
STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS category_name ON categories (name)
