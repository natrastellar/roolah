use super::CurrencyFormat;

pub const USD: CurrencyFormat = CurrencyFormat {
    symbol: "$",
    precision: 2,
    thousand_separator: ",",
    decimal_separator: ".",
};
