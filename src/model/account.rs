use super::{CurrencyRecord, DbDecimal, Tag};
use crate::finance::Currency;
use core::{
    fmt::Display,
    hash::{Hash, Hasher},
};
use rust_decimal::Decimal;
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Account<'a> {
    pub id: i64,
    pub name: String,
    pub currency: CurrencyRecord<'a>,
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
            Currency::new(self.balance, self.currency.format.clone())
        ))
    }
}

impl FromRow<'_, SqliteRow> for Account<'_> {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            currency: CurrencyRecord::from_row(&row)?,
            balance: row.try_get::<DbDecimal, &str>("balance")?.into(),
            posted_balance: row.try_get::<DbDecimal, &str>("posted_balance")?.into(),
            account_type: AccountType::new(
                row.try_get("account_type")?,
                row.try_get("account_type_name")?,
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct AccountType {
    #[sqlx(flatten)]
    pub tag: Tag,
}

impl AccountType {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            tag: Tag { id, name },
        }
    }
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
