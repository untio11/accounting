mod date_deserializer {
    /// Create a serde date deserializer module `local_date_deserializer`
    /// that uses `from_format` to convert from source (e.g.: "%Y%m%d").
    ///
    /// Use via #[serde(with = "local_date_deserializer")]
    #[macro_export]
    macro_rules! date_deserializer_from_format {
        ( $format:expr ) => {
            mod local_date_deserializer {
                use chrono::NaiveDate;
                use serde::{self, Deserialize, Deserializer};

                pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let dt =
                        NaiveDate::parse_from_str(&s, $format).map_err(serde::de::Error::custom)?;

                    Ok(dt)
                }
            }
        };
    }
}
mod direction {
    use serde::{self, de::Error, Deserialize, Deserializer};
    use std::str::FromStr;

    /// Denotes the direction of the transaction.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Direction {
        /// AKA: "Credit", "Bij"
        Incoming,
        /// AKA: "Debit", "Af"
        Outgoing,
    }

    impl FromStr for Direction {
        type Err = String;
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "Af" | "Debit" => Ok(Self::Outgoing),
                "Bij" | "Credit" => Ok(Self::Incoming),
                _ => Err(String::from_str("unknown Direction field").unwrap()),
            }
        }
    }

    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;
            match Direction::from_str(&s) {
                Ok(t) => Ok(t),
                Err(err) => Err(Error::custom(err)),
            }
        }
    }
}
pub mod serde_amount {
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
}
pub mod serde_iban {
    use iban::Iban;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Iban, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        let dt: Iban = Iban::parse(&s).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
    pub fn serialize<S>(iban: &Iban, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(iban.as_str())
    }
}

pub use direction::Direction;
