use miette::{IntoDiagnostic, Result, WrapErr};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous},
    Connection, SqliteConnection,
};

mod account;
mod currency;
mod error;

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

pub async fn init() -> Result<SqliteConnection> {
    let mut conn = create_connection().await.wrap_err("failed to connect")?;
    create_tables(&mut conn)
        .await
        .wrap_err("failed to create tables")?;
    Ok(conn)
}

async fn create_tables(conn: &mut SqliteConnection) -> Result<()> {
    let mut transaction = conn.begin().await.into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_currencies_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_account_types_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_accounts_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_categories_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_methods_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    sqlx::query_file!("src/bin/roolah/database/sql/create_transactions_table.sql")
        .execute(&mut transaction)
        .await
        .into_diagnostic()?;

    transaction.commit().await.into_diagnostic()
}

pub async fn close(conn: SqliteConnection) -> Result<()> {
    // Checkpoints in WAL mode
    conn.close().await.into_diagnostic()
}
