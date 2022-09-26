use miette::{Result, WrapErr};
use roolah::finance::currency::USD;

mod database;

#[tokio::main]
async fn main() -> Result<()> {
    let mut connection = database::init()
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
    for account in &accounts {
        println!("{:?}", account);
    }

    database::close(connection) // Checkpoints in WAL mode
        .await
        .wrap_err("failed to close the database")?;

    Ok(())
}
