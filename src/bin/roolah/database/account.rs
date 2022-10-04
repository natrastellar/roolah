use super::{
    currency,
    tables::{self, AccountTypesColumn, AccountsColumn},
    DatabaseError,
};
use miette::{Context, IntoDiagnostic, Result};
use roolah::{
    finance::CurrencyFormat,
    model::{Account, AccountType},
    ColumnEnum,
};
use rust_decimal::Decimal;
use sqlx::{Connection, SqliteConnection};

pub async fn create_accounts_view(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query_file!("src/bin/roolah/database/sql/create_accounts_with_currency_and_type_view.sql")
        .execute(conn)
        .await
        .map(|_| ())
        .into_diagnostic()
}

pub async fn create_account<'a>(
    conn: &mut SqliteConnection,
    name: &str,
    currency: &CurrencyFormat<'a>,
    account_type: &str,
) -> Result<Account<'a>> {
    let mut transaction = conn.begin().await.into_diagnostic()?;

    let account_type: AccountType = create_account_type(account_type, &mut transaction)
        .await
        .wrap_err("failed to create the account_type")?;
    let currency_id = currency::create_currency(currency, &mut transaction)
        .await
        .wrap_err("failed to create the currency")?;

    let inserted = sqlx::query(&format!(
        "INSERT OR IGNORE INTO {} ({}, {}, {})
        VALUES (?, ?, ?)",
        tables::ACCOUNTS,
        AccountsColumn::Name.name(),
        AccountsColumn::Currency.name(),
        AccountsColumn::AccountType.name(),
    ))
    .bind(&name)
    .bind(currency_id)
    .bind(account_type.tag.id)
    .execute(&mut transaction)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert account with unique name")?;

    if inserted.rows_affected() > 0 {
        transaction
            .commit()
            .await
            .into_diagnostic()
            .wrap_err("failed to commit")?;

        return get_account_by_name(name, conn)
            .await
            .wrap_err("failed to get inserted account");
    }

    transaction
        .rollback()
        .await
        .into_diagnostic()
        .wrap_err("failed to rollback")?;

    let existing_account: Account = get_account_by_name(name, conn).await?;
    if existing_account.currency.id != currency_id
        || existing_account.balance != Decimal::ZERO
        || existing_account.posted_balance != Decimal::ZERO
        || existing_account.account_type != account_type
    {
        return Err(DatabaseError::AccountAlreadyExists(existing_account)).into_diagnostic();
    }

    Ok(existing_account)
}

pub async fn get_account_by_name(
    name: &str,
    conn: &mut SqliteConnection,
) -> Result<Account<'static>> {
    create_accounts_view(conn)
        .await
        .wrap_err("failed to create accounts view")?;

    sqlx::query_as(&format!(
        "SELECT * FROM {} WHERE {} = ?",
        tables::ACCOUNTS_WITH_CURRENCY_AND_TYPE,
        AccountsColumn::Name.name()
    ))
    .bind(&name)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get existing account by name")
}

pub async fn get_all_accounts(conn: &mut SqliteConnection) -> Result<Vec<Account>> {
    create_accounts_view(conn)
        .await
        .wrap_err("failed to create accounts view")?;

    sqlx::query_as(&format!(
        "SELECT * FROM {}",
        tables::ACCOUNTS_WITH_CURRENCY_AND_TYPE
    ))
    .fetch_all(&mut *conn)
    .await
    .into_diagnostic()
}

async fn create_account_type(
    account_type: &str,
    conn: &mut SqliteConnection,
) -> Result<AccountType> {
    let inserted = sqlx::query_as(&format!(
        "INSERT OR IGNORE INTO {} ({})
        VALUES (?)",
        tables::ACCOUNT_TYPES,
        AccountTypesColumn::Name.name()
    ))
    .bind(account_type)
    .fetch_optional(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert account type with unique name")?;

    if let Some(account_type) = inserted {
        return Ok(account_type);
    }

    get_account_type(account_type, conn).await
}

async fn get_account_type(account_type: &str, conn: &mut SqliteConnection) -> Result<AccountType> {
    sqlx::query_as(&format!(
        "SELECT * FROM {} WHERE {} = ?",
        tables::ACCOUNT_TYPES,
        AccountTypesColumn::Name.name()
    ))
    .bind(account_type)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get existing account type by name")
}

//TODO Add tests
