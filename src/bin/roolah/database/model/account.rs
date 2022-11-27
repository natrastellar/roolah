use super::{CurrencyRecord, DbDecimal};
use crate::database::table_identifiers::AccountsWithCurrencyAndTypeColumn;
use core::{
    fmt::Display,
    hash::{Hash, Hasher},
};
use roolah::{finance::Currency, ColumnEnum};
use rust_decimal::Decimal;
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::fmt::{self, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{name}: {balance}",
            name = self.name,
            balance = Currency::new(self.balance, self.currency.format.clone())
        )
    }
}

impl FromRow<'_, SqliteRow> for Account<'_> {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(AccountsWithCurrencyAndTypeColumn::Id.name())?,
            name: row.try_get(AccountsWithCurrencyAndTypeColumn::Name.name())?,
            currency: CurrencyRecord::from_row(row)?,
            balance: row
                .try_get::<DbDecimal, &str>(AccountsWithCurrencyAndTypeColumn::Balance.name())?
                .into(),
            posted_balance: row
                .try_get::<DbDecimal, &str>(
                    AccountsWithCurrencyAndTypeColumn::PostedBalance.name(),
                )?
                .into(),
            account_type: AccountType {
                id: row.try_get(AccountsWithCurrencyAndTypeColumn::AccountTypeId.name())?,
                name: row.try_get(AccountsWithCurrencyAndTypeColumn::AccountTypeName.name())?,
            },
        })
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct AccountType {
    pub id: i64,
    pub name: String,
}

impl PartialEq for AccountType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AccountType {}

impl Hash for AccountType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

//TODO Add tests
