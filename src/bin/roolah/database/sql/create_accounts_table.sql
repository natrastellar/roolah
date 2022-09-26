CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    name TEXT
        UNIQUE
        NOT NULL
        CHECK (name != ''),
    currency INTEGER
        NOT NULL
        REFERENCES currencies(id)
        ON DELETE RESTRICT,
    balance TEXT
        NOT NULL
        DEFAULT '0'
        CHECK (balance != ''),
    posted_balance TEXT
        NOT NULL
        DEFAULT '0'
        CHECK (posted_balance != ''),
    account_type INTEGER
        NOT NULL
        REFERENCES account_types(id)
        ON DELETE RESTRICT
        CHECK (account_type != '')
)
STRICT