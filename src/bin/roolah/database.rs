use miette::{IntoDiagnostic, Result, WrapErr};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous},
    Connection, SqliteConnection,
};
use std::path::Path;

mod account;
mod currency;
mod error;
mod model;
mod schema;
mod table_identifiers;
mod transaction;
mod utils;

pub use account::{create_account, get_account_by_name, get_all_accounts};
pub use error::Error as DatabaseError;
pub use transaction::{create_transaction, TransactionArgs};

async fn create_connection(file: impl AsRef<Path>) -> Result<SqliteConnection> {
    let options = SqliteConnectOptions::new()
            .filename(file)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal) // Faster (no network file support)
            .locking_mode(SqliteLockingMode::Exclusive) // Faster + prevents other app access + allows Wal to work on a VFS without shared-memory primitives
            .synchronous(SqliteSynchronous::Normal) // Safe with Wal, might rollback after power loss
            ;
    SqliteConnection::connect_with(&options)
        .await
        .into_diagnostic()
}

pub async fn init(file: impl AsRef<Path>, clear: bool) -> Result<SqliteConnection> {
    let mut conn = create_connection(file)
        .await
        .wrap_err("failed to connect")?;
    if clear {
        schema::drop_tables(&mut conn).await?;
    }
    schema::create_tables(&mut conn)
        .await
        .wrap_err("failed to create tables")?;
    Ok(conn)
}

pub async fn close(conn: SqliteConnection) -> Result<()> {
    // Checkpoints in WAL mode
    conn.close().await.into_diagnostic()
}

//TODO Add tests
