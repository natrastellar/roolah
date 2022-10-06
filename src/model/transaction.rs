use super::{super::finance::Currency, account::Account, tag::Tag};
use rust_decimal::Decimal;
use sqlx::FromRow;
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{amount} {debit} -> {credit} on {date}",
            amount = self.amount,
            debit = self.debit_account,
            credit = self.credit_account,
            date = self.date
        )?;
        if self.description.is_empty() {
            return write!(f, r#", "{}""#, self.description);
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)
    }
}

//TODO Add tests
