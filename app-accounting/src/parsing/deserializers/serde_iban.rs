use iban::Iban;
use serde::{self, Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Iban, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let dt: Iban = Iban::parse(&s).map_err(serde::de::Error::custom)?;
    Ok(dt)
}
