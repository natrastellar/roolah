use crate::database::table_identifiers::TransactionsWithCategoryAndMethodColumn;
use super::DbDecimal;
use roolah::ColumnEnum;
use rust_decimal::Decimal;
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::{
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};
use time::Date;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: i64,
    pub date: Date,
    pub posted_date: Option<Date>,
    pub category: Option<Category>,
    pub amount: Decimal,
    pub debit_account: Option<i64>,
    pub credit_account: Option<i64>,
    pub authority: String,
    pub description: String,
    pub method: Option<Method>,
    pub check_number: Option<u32>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{amount} on {date}",
            amount = self.amount,
            date = self.date
        )?;
        if self.description.is_empty() {
            return write!(f, r#", "{}""#, self.description);
        }
        Ok(())
    }
}

impl FromRow<'_, SqliteRow> for Transaction {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let category_id: Option<i64> =
            row.try_get(TransactionsWithCategoryAndMethodColumn::CategoryId.name())?;
        let category_name: Option<String> =
            row.try_get(TransactionsWithCategoryAndMethodColumn::CategoryName.name())?;
        let method_id: Option<i64> =
            row.try_get(TransactionsWithCategoryAndMethodColumn::MethodId.name())?;
        let method_name: Option<String> =
            row.try_get(TransactionsWithCategoryAndMethodColumn::MethodName.name())?;
        Ok(Self {
            id: row.try_get(TransactionsWithCategoryAndMethodColumn::Id.name())?,
            date: row.try_get(TransactionsWithCategoryAndMethodColumn::Date.name())?,
            posted_date: row.try_get(TransactionsWithCategoryAndMethodColumn::PostedDate.name())?,
            category: match (category_id, category_name) {
                (Some(id), Some(name)) => Some(Category { id, name }),
                _ => None,
            },
            amount: row
                .try_get::<DbDecimal, &str>(TransactionsWithCategoryAndMethodColumn::Amount.name())?
                .into(),
            debit_account: row
                .try_get(TransactionsWithCategoryAndMethodColumn::DebitAccount.name())?,
            credit_account: row
                .try_get(TransactionsWithCategoryAndMethodColumn::CreditAccount.name())?,
            authority: row.try_get(TransactionsWithCategoryAndMethodColumn::Authority.name())?,
            description: row
                .try_get(TransactionsWithCategoryAndMethodColumn::Description.name())?,
            method: match (method_id, method_name) {
                (Some(id), Some(name)) => Some(Method { id, name }),
                _ => None,
            },
            check_number: row
                .try_get(TransactionsWithCategoryAndMethodColumn::CheckNumber.name())?,
        })
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Category {
    #[sqlx(rename = "category_id")]
    pub id: i64,
    #[sqlx(rename = "category_name")]
    pub name: String,
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Category {}

impl Hash for Category {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Method {
    #[sqlx(rename = "method_id")]
    pub id: i64,
    #[sqlx(rename = "method_name")]
    pub name: String,
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Method {}

impl Hash for Method {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

//TODO Add tests
