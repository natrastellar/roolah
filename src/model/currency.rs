use rust_decimal::Decimal;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

pub mod currencies;
pub use currencies::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurrencyFormat<'a> {
    symbol: &'a str,
    precision: u8,
    thousand_separator: &'a str,
    decimal_separator: &'a str,
}

impl<'a> CurrencyFormat<'a> {
    pub fn format_value<T: Into<Decimal>>(&self, value: T) -> String {
        let value: Decimal = value.into();
        let value_str = format!("{:.*}", self.precision as usize, value.abs());
        if value == Decimal::ZERO {
            return value_str.replace(".", self.decimal_separator);
        }
        let mut parts = value_str.split('.');
        let whole_str = parts.next().expect("leading decimals");
        let decimal_str = parts.next().expect("trailing decimals");
        let mut result = String::new();
        for (i, ch) in whole_str.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 {
                result.push_str(self.thousand_separator);
            }
            result.push(ch);
        }
        result.push_str(self.decimal_separator);
        result.push_str(decimal_str);
        result
    }

    pub fn from<T>(&self, val: T) -> Currency<'a, T> {
        Currency::new(val, *self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Currency<'a, T> {
    value: T,
    format: CurrencyFormat<'a>,
}

impl<'a, T> Currency<'a, T> {
    pub fn new(value: T, format: CurrencyFormat<'a>) -> Self {
        Self { value, format }
    }
}

impl<T> Add for Currency<'_, T>
where
    T: Add + From<<T as Add>::Output>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let val = T::from(self.value + rhs.value);
        Self {
            value: val,
            format: self.format,
        }
    }
}

impl<T> Sub for Currency<'_, T>
where
    T: Sub + From<<T as Sub>::Output>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let val = T::from(self.value - rhs.value);
        Self {
            value: val,
            format: self.format,
        }
    }
}

impl<T, V> Mul<V> for Currency<'_, T>
where
    T: Mul<V> + From<<T as Mul<V>>::Output>,
{
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        let val = T::from(self.value * rhs);
        Self {
            value: val,
            format: self.format,
        }
    }
}

impl<T, V> Div<V> for Currency<'_, T>
where
    T: Div<V> + From<<T as Div<V>>::Output>,
{
    type Output = Self;

    fn div(self, rhs: V) -> Self::Output {
        let val = T::from(self.value / rhs);
        Self {
            value: val,
            format: self.format,
        }
    }
}

impl<T> Display for Currency<'_, T>
where
    T: Into<Decimal> + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = self.format.symbol;
        let value: Decimal = self.value.into();
        let value_str = self.format.format_value(self.value);
        if !value.is_zero() && value.is_sign_negative() {
            f.write_fmt(format_args!("{symbol} ({value_str})"))
        } else {
            f.write_fmt(format_args!("{symbol} {value_str}"))
        }
    }
}

mod test {
    #[test]
    fn math() {
        use super::USD;

        assert_eq!(USD.from(2) + USD.from(3), USD.from(5));
        assert_eq!(USD.from(2) - USD.from(3), USD.from(-1));
        assert_eq!(USD.from(2) * 2, USD.from(4));
        assert_eq!(USD.from(2) / 2, USD.from(1));
    }

    #[test]
    fn format_usd() {
        use super::USD;
        use rust_decimal::Decimal;
        use rust_decimal_macros::dec;

        assert_eq!(USD.format_value(dec!(-3.14)), "3.14");
        assert_eq!(format!("{}", USD.from(2)), "$ 2.00");
        assert_eq!(format!("{}", USD.from(-2)), "$ (2.00)");
        assert_eq!(format!("{}", USD.from(0)), "$ 0.00");
        assert_eq!(format!("{}", USD.from(-Decimal::ZERO)), "$ 0.00");
        assert_eq!(format!("{}", USD.from(dec!(2.124))), "$ 2.12");
        assert_eq!(format!("{}", USD.from(dec!(2.125))), "$ 2.12");
        assert_eq!(format!("{}", USD.from(dec!(-2.125))), "$ (2.12)");
        assert_eq!(format!("{}", USD.from(dec!(2.126))), "$ 2.13");
    }
}
