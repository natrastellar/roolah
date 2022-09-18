use crate::currency::currencies::USD;

mod currency;

fn main() {
    let amount = USD.from(5);
    println!("My amount: {amount}");
}
