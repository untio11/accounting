// const FORMAT: &'static str = "%Y%m%d";

/// date_deserializer::from_format!(format: &str) -> deserialize<>() -> Result<NaiveDate, _>
/// that uses format to convert from source (e.g.: "%Y%m%d").
#[macro_export]
macro_rules! from_format {
    ( $format:expr ) => {
        mod local_date_deserializer {
            use chrono::NaiveDate;
            use serde::{self, Deserialize, Deserializer};

            pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                println!("Received raw date {}, using format {}", s, $format);
                let dt =
                    NaiveDate::parse_from_str(&s, $format).map_err(serde::de::Error::custom)?;
                Ok(dt)
            }
        }
    };
}
