use crate::finance::CurrencyFormat;
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use std::hash::{Hash, Hasher};

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
            .try_get::<i64, &str>("currency_id")
            .or_else(|_| row.try_get::<i64, &str>("id"))?;
        Ok(Self {
            id,
            format: CurrencyFormat::from_row(&row)?,
        })
    }
}

impl FromRow<'_, SqliteRow> for CurrencyFormat<'_> {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            symbol: row.try_get::<String, &str>("symbol")?.into(),
            name: row
                .try_get::<String, &str>("currency_name")
                .or_else(|_| row.try_get::<String, &str>("name"))?
                .into(),
            precision: row.try_get("precision")?,
            thousand_separator: row.try_get::<String, &str>("thousand_separator")?.into(),
            decimal_separator: row.try_get::<String, &str>("decimal_separator")?.into(),
        })
    }
}

//TODO Add tests
