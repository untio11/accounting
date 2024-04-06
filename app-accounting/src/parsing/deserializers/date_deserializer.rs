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
