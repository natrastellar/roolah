use super::{
    model::CurrencyRecord,
    table_identifiers::{self, CurrenciesColumn},
    DatabaseError,
};
use miette::{Context, IntoDiagnostic, Result};
use roolah::finance::CurrencyFormat;
use sqlx::SqliteConnection;

pub async fn create_currency(
    conn: &mut SqliteConnection,
    currency: &CurrencyFormat<'_>,
) -> Result<i64> {
    let inserted = sqlx::query_scalar(&format!(
        r#"INSERT OR IGNORE INTO {currencies} ({symbol}, {name}, {precision}, {thousand_separator}, {decimal_separator})
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            {id} as "{id}!"
        "#,
        currencies = table_identifiers::CURRENCIES,
        symbol = CurrenciesColumn::Symbol,
        name = CurrenciesColumn::Name,
        precision = CurrenciesColumn::Precision,
        thousand_separator = CurrenciesColumn::ThousandSeparator,
        decimal_separator = CurrenciesColumn::DecimalSeparator,
        id = CurrenciesColumn::Id
    ))
    .bind(&currency.symbol)
    .bind(&currency.name)
    .bind(currency.precision)
    .bind(&currency.thousand_separator)
    .bind(&currency.decimal_separator)
    .fetch_optional(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert currency with unique name")?;

    if let Some(id) = inserted {
        return Ok(id);
    }

    let existing: CurrencyRecord = get_currency_by_name(conn, &currency.name).await?;

    if existing.format != *currency {
        return Err(DatabaseError::CurrencyAlreadyExists(existing)).into_diagnostic();
    }

    Ok(existing.id)
}

pub async fn get_currency_by_name(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<CurrencyRecord<'static>> {
    sqlx::query_as(&format!(
        "SELECT * FROM {currencies} WHERE {name} = ?",
        currencies = table_identifiers::CURRENCIES,
        name = CurrenciesColumn::Name
    ))
    .bind(name)
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get existing currency by name")
}

//TODO Add tests
