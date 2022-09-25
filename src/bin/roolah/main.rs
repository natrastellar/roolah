use roolah::model::currency::USD;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let amount = USD.from(5);
    println!("My amount: {amount}");

    Ok(())
}
