use super::DatabaseError;
use miette::{Context, IntoDiagnostic, Result};
use roolah::finance::CurrencyFormat;
use sqlx::SqliteConnection;

pub async fn create_currency(
    currency: &CurrencyFormat<'_>,
    conn: &mut SqliteConnection,
) -> Result<i64> {
    //TODO Reduce duplicate code
    if let Some(inserted) = sqlx::query_scalar!(
        r#"INSERT OR IGNORE INTO currencies (symbol, name, precision, thousand_separator, decimal_separator)
        VALUES (?, ?, ?, ?, ?)
        RETURNING
            id as "id!"
        "#,
        currency.symbol,
        currency.name,
        currency.precision,
        currency.thousand_separator,
        currency.decimal_separator
    )
    .fetch_optional(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert currency with unique name")? {
        return Ok(inserted);
    }

    let (existing_id, existing_currency) =
        sqlx::query!("SELECT * FROM currencies WHERE name = ?", currency.name)
            .fetch_one(conn)
            .await
            .map(|r| -> Result<(i64, CurrencyFormat)> {
                Ok((
                    r.id,
                    CurrencyFormat { //TODO Reduce duplicate code
                        symbol: r.symbol.into(),
                        name: r.name.into(),
                        precision: r
                            .precision
                            .try_into()
                            .into_diagnostic()
                            .wrap_err("precision out of range")?,
                        thousand_separator: r.thousand_separator.into(),
                        decimal_separator: r.decimal_separator.into(),
                    },
                ))
            })
            .into_diagnostic()
            .wrap_err("failed to get existing currency by name")??;

    if existing_currency != *currency {
        return Err(DatabaseError::CurrencyAlreadyExists {
            id: existing_id,
            currency: existing_currency,
        })
        .into_diagnostic();
    }

    Ok(existing_id)
}

//TODO Add tests