use self::tables::{
    AccountTypesColumn, AccountsColumn, CategoriesColumn, CurrenciesColumn, MethodsColumn,
    TransactionsColumn,
};
use miette::{IntoDiagnostic, Result, WrapErr};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous},
    Connection, SqliteConnection,
};

mod account;
mod currency;
mod error;
mod tables;

pub use account::*;
pub use error::Error as DatabaseError;

async fn create_connection() -> Result<SqliteConnection> {
    let options = SqliteConnectOptions::new()
            .filename("roolah.db") //TODO user configurable?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal) // Faster (no network file support)
            .locking_mode(SqliteLockingMode::Exclusive) // Faster + prevents other app access + allows Wal to work on a VFS without shared-memory primitives
            .synchronous(SqliteSynchronous::Normal) // Safe with Wal, might rollback after power loss
            ;
    SqliteConnection::connect_with(&options)
        .await
        .into_diagnostic()
}

macro_rules! drop_existing_tables {
    ($($table:expr),*) => {
        (|| {
            let mut s = String::new();
            $(
                s += "DROP TABLE IF EXISTS ";
                s += $table;
                s += ";";
            )*
            s
        })()
    };
}

pub async fn init(clear: bool) -> Result<SqliteConnection> {
    let mut conn = create_connection().await.wrap_err("failed to connect")?;
    if clear {
        sqlx::query(&drop_existing_tables!(
            tables::TRANSACTIONS,
            tables::ACCOUNTS,
            tables::ACCOUNT_TYPES,
            tables::CURRENCIES,
            tables::CATEGORIES,
            tables::METHODS
        ))
        .execute(&mut conn)
        .await
        .into_diagnostic()
        .wrap_err("failed to drop tables")?;
    }
    create_tables(&mut conn)
        .await
        .wrap_err("failed to create tables")?;
    Ok(conn)
}

async fn create_tables(conn: &mut SqliteConnection) -> Result<()> {
    let mut transaction = conn.begin().await.into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {currencies} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {symbol} TEXT
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != ''),
            {precision} INTEGER
                NOT NULL
                DEFAULT 2,
            {thousand_separator} TEXT
                NOT NULL
                DEFAULT ',',
            {decimal_separator} TEXT
                NOT NULL
                DEFAULT '.'
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS currency_name ON {currencies} ({name})",
        currencies = tables::CURRENCIES,
        id = CurrenciesColumn::Id,
        symbol = CurrenciesColumn::Symbol,
        name = CurrenciesColumn::Name,
        precision = CurrenciesColumn::Precision,
        thousand_separator = CurrenciesColumn::ThousandSeparator,
        decimal_separator = CurrenciesColumn::DecimalSeparator,
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {account_types} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULLs
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS account_type_name ON {account_types} ({name})",
        account_types = tables::ACCOUNT_TYPES,
        id = AccountTypesColumn::Id,
        name = AccountTypesColumn::Name
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {accounts} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != ''),
            {currency} INTEGER
                NOT NULL
                REFERENCES {currencies}({currency_id})
                ON DELETE RESTRICT,
            {balance} TEXT
                NOT NULL
                DEFAULT '0'
                CHECK ({balance} != ''),
            {posted_balance} TEXT
                NOT NULL
                DEFAULT '0'
                CHECK ({posted_balance} != ''),
            {account_type} INTEGER
                NOT NULL
                REFERENCES {account_types}({account_type_id})
                ON DELETE RESTRICT
                CHECK ({account_type} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS account_name ON {accounts} ({name})",
        accounts = tables::ACCOUNTS,
        id = AccountsColumn::Id,
        name = AccountsColumn::Name,
        currency = AccountsColumn::Currency,
        currencies = tables::CURRENCIES,
        currency_id = CurrenciesColumn::Id,
        balance = AccountsColumn::Balance,
        posted_balance = AccountsColumn::PostedBalance,
        account_type = AccountsColumn::AccountType,
        account_types = tables::ACCOUNT_TYPES,
        account_type_id = AccountTypesColumn::Id
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {categories} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS category_name ON {categories} ({name})",
        categories = tables::CATEGORIES,
        id = CategoriesColumn::Id,
        name = CategoriesColumn::Name,
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {methods} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {name} TEXT
                UNIQUE
                NOT NULL
                CHECK ({name} != '')
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS method_name ON {methods} ({name})",
        methods = tables::METHODS,
        id = MethodsColumn::Id,
        name = MethodsColumn::Name,
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {transactions} (
            {id} INTEGER
                PRIMARY KEY
                NOT NULL,
            {date} TEXT
                NOT NULL
                DEFAULT CURRENT_DATE
                CHECK ({date} != ''),
            {posted_date} TEXT
                CHECK ({posted_date} != '')
                CHECK ({posted_date} IS NULL
                    OR strftime('%s', {posted_date}) >= strftime('%s', {date})
                ),
            {category} INTEGER
                REFERENCES {categories}({category_id})
                ON DELETE SET NULL,
            {amount} TEXT
                NOT NULL
                CHECK ({amount} != ''),
            {debit_account} INTEGER
                REFERENCES {accounts}({account_id})
                ON DELETE SET NULL,
            {credit_account} INTEGER
                REFERENCES {accounts}({account_id})
                ON DELETE SET NULL,
            {authority} TEXT
                NOT NULL,
            {description} TEXT
                NOT NULL,
            {method} INTEGER
                REFERENCES {methods}({method_id})
                ON DELETE SET NULL,
            {check_number} INTEGER
                CHECK ({check_number} IS NULL
                    OR ({debit_account} NOT NULL)
                ),
            UNIQUE ({check_number}, {debit_account})
        )
        STRICT;
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_date ON {transactions} ({date});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_posted_date ON {transactions} ({posted_date});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_category ON {transactions} ({category});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_amount ON {transactions} ({amount});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account ON {transactions} ({debit_account});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_credit_account ON {transactions} ({credit_account});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_authority ON {transactions} ({authority});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_description ON {transactions} ({description});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_method ON {transactions} ({method});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_check_number ON {transactions} ({check_number});
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_debit_account_change_magnitude ON {transactions} ({debit_account}, abs({amount}));
        CREATE UNIQUE INDEX IF NOT EXISTS transaction_category_change_magnitude ON {transactions} ({category}, abs({amount}))",
        transactions = tables::TRANSACTIONS,
        id = TransactionsColumn::Id,
        date = TransactionsColumn::Date,
        posted_date = TransactionsColumn::PostedDate,
        category = TransactionsColumn::Category,
        categories = tables::CATEGORIES,
        category_id = CategoriesColumn::Id,
        amount = TransactionsColumn::Amount,
        debit_account = TransactionsColumn::DebitAccount,
        accounts = tables::ACCOUNTS,
        account_id = AccountsColumn::Id,
        credit_account = TransactionsColumn::CreditAccount,
        authority = TransactionsColumn::Authority,
        description = TransactionsColumn::Description,
        method = TransactionsColumn::Method,
        methods = tables::METHODS,
        method_id = MethodsColumn::Id,
        check_number = TransactionsColumn::CheckNumber,
    ))
    .execute(&mut transaction)
    .await
    .into_diagnostic()?;

    transaction.commit().await.into_diagnostic()
}

pub async fn close(conn: SqliteConnection) -> Result<()> {
    // Checkpoints in WAL mode
    conn.close().await.into_diagnostic()
}
