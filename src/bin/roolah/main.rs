use roolah::model::currency::USD;

mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = database::init().await?;

    let amount = USD.from(5);
    println!("My amount: {amount}");

    database::shutdown(connection).await?; // Checkpoints in WAL mode

    Ok(())
}
