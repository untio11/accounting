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
            _ => Err(String::from_str("unknown Direction field: {input}").unwrap()),
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
