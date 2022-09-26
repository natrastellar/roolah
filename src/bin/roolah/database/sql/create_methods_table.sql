CREATE TABLE IF NOT EXISTS methods (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != '')
)
STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS method_name ON methods (name)
