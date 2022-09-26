CREATE TABLE IF NOT EXISTS account_types (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != '')
)
STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS account_type_name ON account_types (name)