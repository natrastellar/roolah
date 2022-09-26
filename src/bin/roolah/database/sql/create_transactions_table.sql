CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER
        PRIMARY KEY
        NOT NULL,
    date TEXT
        NOT NULL
        DEFAULT CURRENT_DATE
        CHECK (date != ''),
    posted_date TEXT
        CHECK (posted_date != '')
        CHECK (posted_date IS NULL
            OR strftime('%s', posted_date) >= strftime('%s', date)
        ),
    category INTEGER
        REFERENCES categories(id)
        ON DELETE SET NULL,
    amount TEXT
        NOT NULL
        CHECK (amount != ''),
    debit_account INTEGER
        REFERENCES accounts(id)
        ON DELETE SET NULL,
    credit_account INTEGER
        REFERENCES accounts(id)
        ON DELETE SET NULL,
    authority TEXT
        NOT NULL,
    description TEXT
        NOT NULL,
    method INTEGER
        REFERENCES methods(id)
        ON DELETE SET NULL,
    check_number INTEGER
        CHECK (check_number IS NULL
            OR (debit_account NOT NULL)
        ),
    UNIQUE (check_number, debit_account)
)
STRICT