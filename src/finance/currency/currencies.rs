use super::CurrencyFormat;
use std::borrow::Cow;

pub const USD: CurrencyFormat = CurrencyFormat {
    symbol: Cow::Borrowed("$"),
    name: Cow::Borrowed("U.S. Dollar"),
    precision: 2,
    thousand_separator: Cow::Borrowed(","),
    decimal_separator: Cow::Borrowed("."),
};
