use super::{currency::create_currency, DatabaseError};
use miette::{Context, IntoDiagnostic, Result};
use roolah::{
    finance::CurrencyFormat,
    model::{Account, AccountType, Tag},
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
    let currency_id = create_currency(currency, &mut transaction)
        .await
        .wrap_err("failed to create the currency")?;

    //TODO Reduce duplicate code
    if let Some(inserted) = sqlx::query!(
        r#"INSERT OR IGNORE INTO accounts (name, currency, account_type)
        VALUES (?, ?, ?)
        RETURNING
            id as "id!",
            balance as "balance!",
            posted_balance as "posted_balance!"
        "#,
        name,
        currency_id,
        account_type.tag.id
    )
    .fetch_optional(&mut transaction)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert account with unique name")?
    {
        transaction
            .commit()
            .await
            .into_diagnostic()
            .wrap_err("failed to commit")?;

        return Ok(Account { //TODO Reduce duplicate code
            id: inserted.id,
            name: name.to_string(),
            currency: currency.clone(),
            balance: inserted //TODO Reduce duplicate code
                .balance
                .parse::<Decimal>()
                .into_diagnostic()
                .wrap_err("failed to parse balance")?,
            posted_balance: inserted //TODO Reduce duplicate code
                .posted_balance
                .parse::<Decimal>()
                .into_diagnostic()
                .wrap_err("failed to parse posted_balance")?,
            account_type,
        });
    }

    transaction
        .rollback()
        .await
        .into_diagnostic()
        .wrap_err("failed to rollback")?;

    create_accounts_view(conn)
        .await
        .wrap_err("failed to create accounts view")?;

    let (existing_currency_id, existing_account) = sqlx::query!(
        "SELECT * FROM accounts_with_currency_and_type WHERE name = ?",
        name
    )
    .fetch_one(conn)
    .await
    .map(|r| -> Result<(i64, Account)> {
        Ok((
            r.currency,
            Account { //TODO Reduce duplicate code
                id: r.id,
                name: r.name.to_string(),
                currency: CurrencyFormat { //TODO Reduce duplicate code
                    symbol: r.symbol.into(),
                    name: r.currency_name.into(),
                    precision: r
                        .precision
                        .try_into()
                        .into_diagnostic()
                        .wrap_err("precision out of range")?,
                    thousand_separator: r.thousand_separator.into(),
                    decimal_separator: r.decimal_separator.into(),
                },
                balance: r //TODO Reduce duplicate code
                    .balance
                    .parse::<Decimal>()
                    .into_diagnostic()
                    .wrap_err("failed to parse balance")?,
                posted_balance: r //TODO Reduce duplicate code
                    .posted_balance
                    .parse::<Decimal>()
                    .into_diagnostic()
                    .wrap_err("failed to parse posted_balance")?,
                account_type: Tag { //TODO Reduce duplicate code
                    id: r.account_type,
                    name: r.account_type_name,
                }
                .into(),
            },
        ))
    })
    .into_diagnostic()
    .wrap_err("failed to get existing account by name")??;

    if existing_currency_id != currency_id
        || existing_account.balance != Decimal::ZERO
        || existing_account.posted_balance != Decimal::ZERO
        || existing_account.account_type != account_type
    {
        return Err(DatabaseError::AccountAlreadyExists(existing_account)).into_diagnostic();
    }

    Ok(existing_account)
}

async fn create_account_type(
    account_type: &str,
    conn: &mut SqliteConnection,
) -> Result<AccountType> {
    //TODO skip Tag once query_as! supports FromRow: https://github.com/launchbadge/sqlx/issues/514
    //TODO remove type override once 0.7 is released: https://github.com/launchbadge/sqlx/issues/1923
    //TODO Reduce duplicate code
    if let Some(inserted) = sqlx::query_as!(
        Tag,
        r#"INSERT OR IGNORE INTO account_types (name)
        VALUES (?)
        RETURNING
            id AS "id!",
            name AS "name!"
        "#,
        account_type
    )
    .fetch_optional(&mut *conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to insert account type with unique name")?
    {
        return Ok(inserted.into());
    }

    sqlx::query_as!(
        Tag,
        "SELECT * FROM account_types WHERE name = ?",
        account_type
    )
    .fetch_one(conn)
    .await
    .into_diagnostic()
    .wrap_err("failed to get existing account type by name")
    .map(AccountType::from)
}

pub async fn get_all_accounts(conn: &mut SqliteConnection) -> Result<Vec<Account>> {
    create_accounts_view(conn)
        .await
        .wrap_err("failed to create accounts view")?;

    sqlx::query!("SELECT * FROM accounts_with_currency_and_type")
        .fetch_all(&mut *conn)
        .await
        .map(|records| {
            //TODO implement FromRow for Account/CurrencyFormat once query_as! supports it?: https://github.com/launchbadge/sqlx/issues/514
            records
                .into_iter()
                .map(|r| -> Result<Account> {
                    Ok(Account {
                        id: r.id,
                        name: r.name,
                        currency: CurrencyFormat { //TODO Reduce duplicate code
                            symbol: r.symbol.into(),
                            name: r.currency_name.into(),
                            precision: r
                                .precision
                                .try_into()
                                .into_diagnostic()
                                .wrap_err("precision out of range")?,
                            thousand_separator: r.thousand_separator.into(),
                            decimal_separator: r.decimal_separator.into(),
                        },
                        balance: r //TODO Reduce duplicate code
                            .balance
                            .parse::<Decimal>()
                            .into_diagnostic()
                            .wrap_err("failed to parse balance")?,
                        posted_balance: r //TODO Reduce duplicate code
                            .posted_balance
                            .parse::<Decimal>()
                            .into_diagnostic()
                            .wrap_err("failed to parse posted_balance")?,
                        account_type: Tag { //TODO Reduce duplicate code
                            id: r.account_type,
                            name: r.account_type_name,
                        }
                        .into(),
                    })
                })
                .collect()
        })
        .into_diagnostic()?
}

//TODO Add tests