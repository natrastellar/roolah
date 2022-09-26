use std::borrow::Cow;

use miette::{Result, WrapErr};
use roolah::finance::currency::USD;

mod database;

#[tokio::main]
async fn main() -> Result<()> {
    const RECREATE_DATABASE: bool = true;
    let mut connection = database::init(RECREATE_DATABASE)
        .await
        .wrap_err("failed to initialize the database")?;

    let checking_account =
        database::create_account(&mut connection, "My Checking", &USD, "Checking")
            .await
            .wrap_err("failed to create a checking account")?;
    let accounts = database::get_all_accounts(&mut connection)
        .await
        .wrap_err("failed to get accounts")?;
    assert_eq!(1, accounts.len());
    assert_eq!(Some(&checking_account), accounts.first());

    let checking_account =
        database::create_account(&mut connection, "My Checking", &USD, "Savings").await;
    assert!(checking_account.is_err());

    let mut cad = USD.into_owned();
    cad.name = Cow::Borrowed("Canadian Dollar");
    let checking_account =
        database::create_account(&mut connection, "My Checking", &cad, "Checking").await;
    assert!(checking_account.is_err());

    database::close(connection) // Checkpoints in WAL mode
        .await
        .wrap_err("failed to close the database")?;

    Ok(())
}
