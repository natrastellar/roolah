use rust_decimal::Decimal;
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

pub struct DbDecimal(pub Decimal);

impl From<Decimal> for DbDecimal {
    fn from(dec: Decimal) -> Self {
        Self(dec)
    }
}

impl Into<Decimal> for DbDecimal {
    fn into(self) -> Decimal {
        self.0
    }
}

impl Type<Sqlite> for DbDecimal {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for DbDecimal {
    fn encode_by_ref(&self, args: &mut Vec<SqliteArgumentValue<'_>>) -> IsNull {
        let value = self.0.to_string();
        value.encode(args)
    }
}

impl Decode<'_, Sqlite> for DbDecimal {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<'_, Sqlite>>::decode(value)?;
        let dec = value.parse::<Decimal>()?;
        Ok(Self::from(dec))
    }
}

//TODO Add tests
