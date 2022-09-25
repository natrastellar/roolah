use std::fmt::Display;
use std::hash::{Hash, Hasher};

use rust_decimal::Decimal;
use time::Date;

use super::account::Account;
use super::currency::Currency;
use super::tag::Tag;

#[derive(Debug, Clone)]
pub struct Transaction {
    id: u32,
    date: Date,
    posted_date: Option<Date>,
    category: Tag,
    amount: Currency<'static, Decimal>,
    debit_account: Account,
    credit_account: Account,
    description: String,
    method: Tag,
    check_number: Option<u64>,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Transaction {}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} -> {} on {}",
            self.amount, self.debit_account, self.credit_account, self.date
        ))?;
        if self.description.is_empty() {
            return f.write_fmt(format_args!(", \"{}\"", self.description));
        }
        Ok(())
    }
}
