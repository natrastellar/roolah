use crate::database::TransactionArgs;
use miette::{Result, WrapErr};
use roolah::finance::currency::USD;
use rust_decimal_macros::dec;
use std::borrow::Cow;
use time::macros::date;

mod database;

#[tokio::main]
async fn main() -> Result<()> {
    const DATABASE_FILE: &str = "roolah.db"; //TODO user configurable? embed in the file? use as the file?
    const RECREATE_DATABASE: bool = true;
    let mut conn = database::init(DATABASE_FILE, RECREATE_DATABASE)
        .await
        .wrap_err("failed to initialize the database")?;

    let checking_account = database::create_account(&mut conn, "My Checking", &USD, "Checking")
        .await
        .wrap_err("failed to create a checking account")?;
    assert_eq!(checking_account.name, "My Checking");
    assert_eq!(checking_account.currency.format.symbol, "$");
    assert_eq!(checking_account.currency.format.name, "U.S. Dollar");
    assert_eq!(checking_account.account_type.name, "Checking");

    let accounts = database::get_all_accounts(&mut conn)
        .await
        .wrap_err("failed to get accounts")?;
    assert_eq!(1, accounts.len());
    assert_eq!(Some(&checking_account), accounts.first());

    let savings_account = database::create_account(&mut conn, "My Savings", &USD, "Savings")
        .await
        .wrap_err("failed to create a savings account")?;

    let checking_account_as_savings =
        database::create_account(&mut conn, "My Checking", &USD, "Savings").await;
    assert!(checking_account_as_savings.is_err());

    let mut cad = USD.into_owned();
    cad.name = Cow::Borrowed("Canadian Dollar");
    let canadian_checking_account =
        database::create_account(&mut conn, "My Checking", &cad, "Checking").await;
    assert!(canadian_checking_account.is_err());

    let mut args = TransactionArgs::new(
        date!(2022 - 10 - 6),
        dec!(5.00),
        checking_account.id,
        savings_account.id,
        "transfer",
    );
    args.description = "deposit";
    let transfer = database::create_transaction(&mut conn, args)
        .await
        .wrap_err("failed to create a transfer")?;
    assert_eq!(transfer.date, date!(2022 - 10 - 6));
    assert_eq!(transfer.posted_date, None);
    assert_eq!(transfer.category, None);
    assert_eq!(transfer.amount, dec!(5.00));
    assert_eq!(transfer.debit_account, Some(checking_account.id));
    assert_eq!(transfer.credit_account, Some(savings_account.id));
    assert_eq!(transfer.authority, "");
    assert_eq!(transfer.description, "deposit");
    assert_eq!(transfer.method.as_ref().map(|m| m.name.as_ref()), Some("transfer"));

    // let transactions = database::get_transactions_on_date(&mut conn, &date!(2022 - 10 - 6));
    // assert_eq!(transactions.size(), 1);
    // assert_eq!(Some(&transfer), transactions.first());
    // let transactions = database::get_transactions_by_account(&mut conn, checking_account.id);
    // assert_eq!(transactions.size(), 1);
    // assert_eq!(Some(&transfer), transactions.first());
    // let transactions = database::get_transactions_by_account(&mut conn, savings_account.id);
    // assert_eq!(transactions.size(), 1);
    // assert_eq!(Some(&transfer), transactions.first());

    database::close(conn) // Checkpoints in WAL mode
        .await
        .wrap_err("failed to close the database")?;

    Ok(())
}
