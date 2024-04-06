use rust_decimal::Decimal;
use serde::{self, Deserialize, Deserializer};
use std::str::FromStr;

fn to_decimal(input: String) -> Decimal {
    let input_locale: locale::Numeric = locale::Numeric {
        decimal_sep: String::from(","),
        thousands_sep: String::from("."),
    };
    let input = input.replace(&input_locale.decimal_sep, ".");
    match Decimal::from_str(&input) {
        Err(_) => panic!("Can't convert {}", input),
        Ok(value) => value,
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    Ok(to_decimal(s))
}
