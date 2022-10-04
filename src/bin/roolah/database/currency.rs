use super::{
    tables::{self, CurrenciesColumn},
    DatabaseError,
};
use miette::{Context, IntoDiagnostic, Result};
use roolah::{finance::CurrencyFormat, model::CurrencyRecord, ColumnEnum};
use sqlx::SqliteConnection;

pub async fn create_currency(
    currency: &CurrencyFormat<'_>,
    conn: &mut SqliteConnection,
) -> Result<i64> {
    let inserted = sqlx::query_scalar(&format!(
        r#"INSERT OR IGNORE INTO {0} ({1}, {2}, {3}, {4}, {5})
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            {6} as "{6}!"
        "#,
        tables::CURRENCIES,
        CurrenciesColumn::Symbol.name(),
        CurrenciesColumn::Name.name(),
        CurrenciesColumn::Precision.name(),
        CurrenciesColumn::ThousandSeparator.name(),
        CurrenciesColumn::DecimalSeparator.name(),
        CurrenciesColumn::Id.name()
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

    let existing: CurrencyRecord = get_currency_by_name(&currency.name, conn).await?;

    if existing.format != *currency {
        return Err(DatabaseError::CurrencyAlreadyExists(existing)).into_diagnostic();
    }

    Ok(existing.id)
}

pub async fn get_currency_by_name(
    name: &str,
    conn: &mut SqliteConnection,
) -> Result<CurrencyRecord<'static>> {
    sqlx::query_as("SELECT * FROM currencies WHERE name = ?")
        .bind(&name)
        .fetch_one(conn)
        .await
        .into_diagnostic()
        .wrap_err("failed to get existing currency by name")
}

//TODO Add tests
