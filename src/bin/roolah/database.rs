use miette::{IntoDiagnostic, Result, WrapErr};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous},
    Connection, SqliteConnection,
};

mod account;
mod currency;
mod error;
mod schema;
mod table_identifiers;

pub use account::{create_account, get_account_by_name, get_all_accounts};
pub use error::Error as DatabaseError;

async fn create_connection() -> Result<SqliteConnection> {
    const DATABASE_FILE: &str = "roolah.db"; //TODO user configurable? embed in the file? use as the file?
    let options = SqliteConnectOptions::new()
            .filename(DATABASE_FILE)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal) // Faster (no network file support)
            .locking_mode(SqliteLockingMode::Exclusive) // Faster + prevents other app access + allows Wal to work on a VFS without shared-memory primitives
            .synchronous(SqliteSynchronous::Normal) // Safe with Wal, might rollback after power loss
            ;
    SqliteConnection::connect_with(&options)
        .await
        .into_diagnostic()
}

pub async fn init(clear: bool) -> Result<SqliteConnection> {
    let mut conn = create_connection().await.wrap_err("failed to connect")?;
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
