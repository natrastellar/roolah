use rust_decimal::Decimal;
use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use super::currency::{Currency, CurrencyFormat};

#[derive(Debug, Clone)]
pub struct Account {
    id: u32,
    name: String,
    currency: CurrencyFormat<'static>,
    balance: Decimal,
    posted_balance: Decimal,
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Account {}

impl Hash for Account {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.name,
            Currency::new(self.balance, self.currency)
        ))
    }
}
