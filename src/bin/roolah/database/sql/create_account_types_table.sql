CREATE TABLE IF NOT EXISTS account_types (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != '')
)
STRICT