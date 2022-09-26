CREATE TABLE IF NOT EXISTS currencies (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    symbol TEXT
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != ''),
    precision INTEGER
        NOT NULL
        DEFAULT 2,
    thousand_separator TEXT
        NOT NULL
        DEFAULT ',',
    decimal_separator TEXT
        NOT NULL
        DEFAULT '.'
)
STRICT