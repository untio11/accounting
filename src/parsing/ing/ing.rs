use chrono::NaiveDate;
use iban::Iban;
use rust_decimal::Decimal;
use serde::{self, de::Error, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, PartialEq, Deserialize)]
pub enum Code {
    AC,
    BA,
    DV,
    FL,
    GF,
    GM,
    GT,
    IC,
    ID,
    OV,
    PK,
    PO,
    ST,
    VZ,
}

/// Denotes the direction of the transaction.
#[derive(Debug)]
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

crate::from_format!("%Y%m%d");

#[derive(Debug, Deserialize)]
pub struct IngCurrentAccount {
    #[serde(rename = "Date", with = "local_date_deserializer")]
    pub date: NaiveDate,
    // YYYYMMDD -> YYYY-MM-DD
    #[serde(rename = "Name / Description")]
    pub name: String,
    // Name, often not very descriptive.
    #[serde(rename = "Account", with = "serde_iban")]
    pub account: Iban,
    // XX00XXXX0000000000 -> IBAN, make a type for that. Always account of the owner?
    #[serde(rename = "Counterparty")]
    pub counter_party: Option<String>,
    // IBAN, or [A-Z]?[0-9]{8} for ING sub-accounts (sparen, beleggen). Sparen actually empty, gotta extract from Notifications column...
    // Basically needs a post-processing step still. Do this when moving to universal transaction type.
    #[serde(rename = "Code")]
    pub code: Code, // Constant type from see https://nl.wikipedia.org/wiki/Rekeningafschrift

    #[serde(rename = "Debit/credit")]
    pub direction: Direction, // Constant "Debit" | "Credit" - debit = incoming, credit = outgoing.

    #[serde(rename = "Amount (EUR)", with = "serde_amount")]
    pub amount: Decimal, // 0000,00 - Always positive: sign depends on `self.direction`.

    #[serde(rename = "Transaction type")]
    pub transaction_type: String, // Full name of `self.code`.

    #[serde(rename = "Notifications")]
    pub description: String, // Extra description as filled in by the initiator of the transaction.

    #[serde(rename = "Resulting balance", with = "serde_amount")]
    pub balance: Decimal, // 0000,00 - Balance of the account after this transaction

    #[serde(rename = "Tag")]
    pub tags: String, // Extra custom tags and/or text added by the account owner. -> Split to a set of #tags and a rest String.
}

mod serde_iban {
    use iban::Iban;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Iban, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        println!("Received raw iban {}", s);
        let dt: Iban = Iban::parse(&s).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

mod serde_amount {
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
