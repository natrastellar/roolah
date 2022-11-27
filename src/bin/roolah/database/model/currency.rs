use crate::database::table_identifiers::{AccountsWithCurrencyAndTypeColumn, CurrenciesColumn};
use roolah::{finance::CurrencyFormat, ColumnEnum};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::hash::{Hash, Hasher};

pub struct DbCurrencyFormat<'a>(pub CurrencyFormat<'a>);

#[derive(Debug, Clone, Eq)]
pub struct CurrencyRecord<'a> {
    pub id: i64,
    pub format: CurrencyFormat<'a>,
}

impl PartialEq for CurrencyRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for CurrencyRecord<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl FromRow<'_, SqliteRow> for CurrencyRecord<'_> {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        let id = row
            .try_get::<i64, &str>(AccountsWithCurrencyAndTypeColumn::CurrencyId.name())
            .or_else(|_| row.try_get::<i64, &str>(CurrenciesColumn::Id.name()))?;
        Ok(Self {
            id,
            format: DbCurrencyFormat::from_row(row)?.0,
        })
    }
}

impl FromRow<'_, SqliteRow> for DbCurrencyFormat<'_> {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(DbCurrencyFormat(CurrencyFormat {
            symbol: row
                .try_get::<String, &str>(CurrenciesColumn::Symbol.name())?
                .into(),
            name: row
                .try_get::<String, &str>(AccountsWithCurrencyAndTypeColumn::CurrencyName.name())
                .or_else(|_| row.try_get::<String, &str>(CurrenciesColumn::Name.name()))?
                .into(),
            precision: row.try_get(CurrenciesColumn::Precision.name())?,
            thousand_separator: row
                .try_get::<String, &str>(CurrenciesColumn::ThousandSeparator.name())?
                .into(),
            decimal_separator: row
                .try_get::<String, &str>(CurrenciesColumn::DecimalSeparator.name())?
                .into(),
        }))
    }
}

//TODO Add tests
