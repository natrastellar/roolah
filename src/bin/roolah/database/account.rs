use super::{
    currency,
    table_identifiers::{
        self, AccountTypesColumn, AccountsColumn, AccountsWithCurrencyAndTypeColumn,
        CurrenciesColumn,
    },
    DatabaseError,
};
use miette::{Context, IntoDiagnostic, Result};
use roolah::{
    finance::CurrencyFormat,
    model::{Account, AccountType},
};
use rust_decimal::Decimal;
use sqlx::{Connection, SqliteConnection};

pub async fn create_accounts_view(conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query(&format!(
        "CREATE VIEW IF NOT EXISTS {view} AS
        SELECT
            {accounts}.{id} AS {view_id},
            {accounts}.{name} AS {view_name},
            {accounts}.{currency} AS {view_currency_id},
            {accounts}.{balance} AS {view_balance},
            {accounts}.{posted_balance} AS {view_posted_balance},
            {accounts}.{account_type} AS {view_account_type_id},
            {currencies}.{symbol} AS {view_symbol},
            {currencies}.{currency_name} AS {view_currency_name},
            {currencies}.{precision} AS {view_precision},
            {currencies}.{thousand_separator} AS {view_thousand_separator},
            {currencies}.{decimal_separator} AS {view_decimal_separator},
            {account_types}.{account_type_name} as {view_account_type_name}
        FROM {accounts}
        INNER JOIN {account_types}
            ON {accounts}.{account_type} = {account_types}.{account_type_id}
        INNER JOIN {currencies}
            ON {accounts}.{currency} = {currencies}.{currency_id}",
        view = table_identifiers::ACCOUNTS_WITH_CURRENCY_AND_TYPE,
        accounts = table_identifiers::ACCOUNTS,
        id = AccountsColumn::Id,
        view_id = AccountsWithCurrencyAndTypeColumn::Id,
        name = AccountsColumn::Name,
        view_name = AccountsWithCurrencyAndTypeColumn::Name,
        currency = AccountsColumn::Currency,
        view_currency_id = AccountsWithCurrencyAndTypeColumn::CurrencyId,
        balance = AccountsColumn::Balance,
        view_balance = AccountsWithCurrencyAndTypeColumn::Balance,
        posted_balance = AccountsColumn::PostedBalance,
        view_posted_balance = AccountsWithCurrencyAndTypeColumn::PostedBalance,
        account_type = AccountsColumn::AccountType,
        view_account_type_id = AccountsWithCurrencyAndTypeColumn::AccountTypeId,
        currencies = table_identifiers::CURRENCIES,
        symbol = CurrenciesColumn::Symbol,
        view_symbol = AccountsWithCurrencyAndTypeColumn::Symbol,
        currency_name = CurrenciesColumn::Name,
        view_currency_name = AccountsWithCurrencyAndTypeColumn::CurrencyName,
        precision = CurrenciesColumn::Precision,
        view_precision = AccountsWithCurrencyAndTypeColumn::Precision,
        thousand_separator = CurrenciesColumn::ThousandSeparator,
        view_thousand_separator = AccountsWithCurrencyAndTypeColumn::ThousandSeparator,
        decimal_separator = CurrenciesColumn::DecimalSeparator,
        view_decimal_separator = AccountsWithCurrencyAndTypeColumn::DecimalSeparator,
        account_types = table_identifiers::ACCOUNT_TYPES,
        account_type_name = AccountTypesColumn::Name,
        view_account_type_name = AccountsWithCurrencyAndTypeColumn::AccountTypeName,
        account_type_id = AccountTypesColumn::Id,
        currency_id = CurrenciesColumn::Id,
    ))
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

    let account_type: AccountType = create_account_type(&mut transaction, account_type)
        .await
        .wrap_err("failed to create the account_type")?;
    let currency_id = currency::create_currency(&mut transaction, currency)
        .await
        .wrap_err("failed to create the currency")?;

    let inserted = sqlx::query(&format!(
        "INSERT OR IGNORE INTO {accounts} ({name}, {currency}, {account_type})
        VALUES (?, ?, ?)",
        accounts = table_identifiers::ACCOUNTS,
        name = AccountsColumn::Name,
        currency = AccountsColumn::Currency,
        account_type = AccountsColumn::AccountType,
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

        return get_account_by_name(conn, name)
            .await
            .wrap_err("failed to get inserted account");
    }

    transaction
        .rollback()
        .await
        .into_diagnostic()
        .wrap_err("failed to rollback")?;

    let existing_account: Account = get_account_by_name(conn, name).await?;
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
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<Account<'static>> {
    create_accounts_view(conn)
        .await
        .wrap_err("failed to create accounts view")?;

    sqlx::query_as(&format!(
        "SELECT * FROM {accounts_view} WHERE {name} = ?",
        accounts_view = table_identifiers::ACCOUNTS_WITH_CURRENCY_AND_TYPE,
        name = AccountsColumn::Name
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
        table_identifiers::ACCOUNTS_WITH_CURRENCY_AND_TYPE
    ))
    .fetch_all(&mut *conn)
    .await
    .into_diagnostic()
}

async fn create_account_type(
    conn: &mut SqliteConnection,
    account_type: &str,
) -> Result<AccountType> {
    let inserted = sqlx::query_as(&format!(
        "INSERT OR IGNORE INTO {account_types} ({name})
        VALUES (?)",
        account_types = table_identifiers::ACCOUNT_TYPES,
        name = AccountTypesColumn::Name
    ))
    .bind(account_type)
    .fetch_optional(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert account type with unique name")?;

    if let Some(account_type) = inserted {
        return Ok(account_type);
    }

    get_account_type(conn, account_type).await
}

async fn get_account_type(conn: &mut SqliteConnection, account_type: &str) -> Result<AccountType> {
    sqlx::query_as(&format!(
        "SELECT * FROM {account_types} WHERE {name} = ?",
        account_types = table_identifiers::ACCOUNT_TYPES,
        name = AccountTypesColumn::Name
    ))
    .bind(account_type)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get existing account type by name")
}

//TODO Add tests
