use roolah::model::Account;
use sqlx::{
    sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteQueryResult,
        SqliteSynchronous,
    },
    Connection, SqliteConnection,
};

async fn create_connection() -> Result<SqliteConnection, sqlx::Error> {
    let options = SqliteConnectOptions::new()
            .filename("roolah.db") //TODO user configurable?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal) // Faster (no network file support)
            .locking_mode(SqliteLockingMode::Exclusive) // Faster + prevents other app access + allows Wal to work on a VFS without shared-memory primitives
            .synchronous(SqliteSynchronous::Normal) // Safe with Wal, might rollback after power loss
            ;
    SqliteConnection::connect_with(&options).await
}

pub async fn init() -> Result<SqliteConnection, sqlx::Error> {
    let mut conn = create_connection().await?;
    create_tables(&mut conn).await?;
    Ok(conn)
}

async fn create_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let mut transaction = conn.begin().await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS currencies (
            id INTEGER
                PRIMARY KEY,
            symbol TEXT
                NOT NULL,
            name TEXT
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
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS account_types (
            id INTEGER
                PRIMARY KEY,
            name TEXT
                NOT NULL
                CHECK (name != '')
        )
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER
                PRIMARY KEY,
            name TEXT
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
            type TEXT
                NOT NULL
                REFERENCES account_types(id)
                ON DELETE RESTRICT
                CHECK (type != '')
        )
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER
                PRIMARY KEY,
            name TEXT
                NOT NULL
                CHECK (name != '')
        )
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS methods (
            id INTEGER
                PRIMARY KEY,
            name TEXT
                NOT NULL
                CHECK (name != '')
        )
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER
                PRIMARY KEY,
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
        STRICT"
    )
    .execute(&mut transaction)
    .await?;

    transaction.commit().await
}

pub async fn shutdown(conn: SqliteConnection) -> Result<(), sqlx::Error> {
    // Checkpoints in WAL mode
    conn.close().await
}

pub fn query_accounts() -> Result<Vec<Account>, sqlx::Error> {
    todo!()
}
