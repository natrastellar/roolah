use super::{
    super::finance::{Currency, CurrencyFormat},
    Tag,
};
use core::{
    fmt::Display,
    hash::{Hash, Hasher},
};
use std::fmt;
use rust_decimal::Decimal;
use sqlx::FromRow;

#[derive(Debug, Clone)]
pub struct Account<'a> {
    pub id: i64,
    pub name: String,
    pub currency: CurrencyFormat<'a>,
    pub balance: Decimal,
    pub posted_balance: Decimal,
    pub account_type: AccountType,
}

impl PartialEq for Account<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Account<'_> {}

impl Hash for Account<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Account<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}: {}",
            self.name,
            Currency::new(self.balance, self.currency.clone())
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct AccountType {
    #[sqlx(flatten)]
    pub tag: Tag,
}

impl From<Tag> for AccountType {
    fn from(tag: Tag) -> Self {
        Self { tag }
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.tag))
    }
}

//TODO Add tests