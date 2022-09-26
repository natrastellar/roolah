use super::{super::finance::Currency, account::Account, tag::Tag};
use core::{
    fmt::Display,
    hash::{Hash, Hasher},
};
use rust_decimal::Decimal;
use sqlx::FromRow;
use std::fmt;
use time::Date;

#[derive(Debug, Clone)]
pub struct Transaction<'a> {
    pub id: i64,
    pub date: Date,
    pub posted_date: Option<Date>,
    pub category: Tag,
    pub amount: Currency<'a, Decimal>,
    pub debit_account: Account<'a>,
    pub credit_account: Account<'a>,
    pub authority: String,
    pub description: String,
    pub method: Tag,
    pub check_number: Option<u64>,
}

impl PartialEq for Transaction<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Transaction<'_> {}

impl Hash for Transaction<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Transaction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{} {} -> {} on {}",
            self.amount, self.debit_account, self.credit_account, self.date
        ))?;
        if self.description.is_empty() {
            return f.write_fmt(format_args!(r#", "{}""#, self.description));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow)]
pub struct Category {
    #[sqlx(flatten)]
    pub tag: Tag,
}

impl From<Tag> for Category {
    fn from(tag: Tag) -> Self {
        Self { tag }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.tag))
    }
}

//TODO Add tests