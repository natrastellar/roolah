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
STRICT;

CREATE UNIQUE INDEX IF NOT EXISTS transaction_date ON transactions (date);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_posted_date ON transactions (posted_date);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_category ON transactions (category);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_amount ON transactions (amount);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account ON transactions (debit_account);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_credit_account ON transactions (credit_account);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_authority ON transactions (authority);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_description ON transactions (description);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_method ON transactions (method);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_check_number ON transactions (check_number);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account_change_magnitude ON transactions (debit_account, abs(amount));
CREATE UNIQUE INDEX IF NOT EXISTS transaction_category_change_magnitude ON transactions (category, abs(amount))