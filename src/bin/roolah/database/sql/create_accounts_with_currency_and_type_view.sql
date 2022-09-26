CREATE VIEW IF NOT EXISTS accounts_with_currency_and_type AS
SELECT
    accounts.id AS id,
    accounts.name AS name,
    accounts.currency AS currency,
    accounts.balance AS balance,
    accounts.posted_balance AS posted_balance,
    accounts.account_type AS account_type,
    currencies.symbol AS symbol,
    currencies.name AS currency_name,
    currencies.precision AS precision,
    currencies.thousand_separator AS thousand_separator,
    currencies.decimal_separator AS decimal_separator,
    account_types.name as account_type_name
FROM accounts
INNER JOIN account_types
    ON accounts.account_type = account_types.id
INNER JOIN currencies
    ON accounts.currency = currencies.id