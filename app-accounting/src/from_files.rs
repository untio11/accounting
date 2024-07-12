pub mod serializers {
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
                        let dt = NaiveDate::parse_from_str(&s, $format)
                            .map_err(serde::de::Error::custom)?;

                        Ok(dt)
                    }
                }
            };
        }
    }
    mod direction {
        use crate::canonical::transaction::Direction;
        use serde::{self, de::Error, Deserialize, Deserializer};
        use std::str::FromStr;

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

    pub mod ing {
        use super::*;
        use crate::canonical::{account::*, transaction::*};
        use chrono::NaiveDate;
        use iban::Iban;
        use regex::Regex;
        use rust_decimal::Decimal;
        use serde::{self, Deserialize};
        use std::iter::FromIterator;

        #[derive(Debug, PartialEq, Deserialize, Eq, Hash)]
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

        crate::date_deserializer_from_format!("%Y%m%d");

        /// Slightly processed CSV from ING. Raw, use as base to implement
        /// Transaction.
        #[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
        pub struct IngCurrentAccount {
            /// YYYYMMDD -> YYYY-MM-DD
            #[serde(rename = "Date", alias = "Datum", with = "local_date_deserializer")]
            pub date: NaiveDate,

            /// Name, often not very descriptive.
            #[serde(rename = "Name / Description", alias = "Naam / Omschrijving")]
            pub name: String,

            /// XX00XXXX0000000000 -> IBAN, make a type for that. Always account of the owner?
            #[serde(rename = "Account", alias = "Rekening", with = "serde_iban")]
            pub account: Iban,

            // IBAN, or [A-Z]?[0-9]{8} for ING sub-accounts (sparen, beleggen). Sparen actually empty, gotta extract from Notifications column...
            // Basically needs a post-processing step still. Do this when moving to universal transaction type.
            #[serde(rename = "Counterparty", alias = "Tegenrekening")]
            pub counter_party: Option<String>,

            /// Constant type from see https://nl.wikipedia.org/wiki/Rekeningafschrift
            #[serde(rename = "Code")]
            pub code: Code,

            /// Constant "Debit" | "Credit"
            ///
            /// Debit = incoming, Credit = outgoing.
            #[serde(rename = "Debit/credit", alias = "Af Bij")]
            pub direction: Direction,

            /// 0000,00 - Always positive: sign depends on `self.direction`.
            #[serde(rename = "Amount (EUR)", alias = "Bedrag (EUR)", with = "serde_amount")]
            pub amount: Decimal,

            /// Full name of `self.code`.
            #[serde(rename = "Transaction type", alias = "Mutatiesoort")]
            pub transaction_type: String,

            /// Extra description as filled in by the initiator of the transaction.
            #[serde(rename = "Notifications", alias = "Mededelingen")]
            pub description: String,

            /// 0000,00 - Balance of the account after this transaction
            #[serde(
                rename = "Resulting balance",
                alias = "Saldo na mutatie",
                with = "serde_amount"
            )]
            pub balance: Decimal,

            /// Extra custom tags and/or text added by the account
            /// owner. -> Split to a set of #tags and a rest String.
            #[serde(rename = "Tag")]
            pub tags: String,
        }
        impl From<IngCurrentAccount> for Transaction {
            fn from(value: IngCurrentAccount) -> Self {
                Transaction {
                    amount: value.amount,
                    date: value.date,
                    description: String::from(&value.description),
                    inherent_tags: inherent_tags(&value),
                    source: source(&value),
                    sink: sink(&value),
                }
            }
        }

        fn inherent_tags(ing_transaction: &IngCurrentAccount) -> Vec<String> {
            let reg = Regex::new(r"(#.+?)\b").unwrap();
            let mut result = Vec::from_iter(
                reg.find_iter(&ing_transaction.tags)
                    .map(|m| String::from(m.as_str().trim())),
            );
            result.sort_unstable();
            result.dedup();
            result
        }

        fn sink(ing_transaction: &IngCurrentAccount) -> Node {
            match ing_transaction.direction {
                Direction::Incoming => Node::ProperAccount(Account {
                    iban: ing_transaction.account,
                    name: String::from("My Account"),
                }),
                Direction::Outgoing => determine_node_type(ing_transaction),
            }
        }

        fn source(ing_transaction: &IngCurrentAccount) -> Node {
            match ing_transaction.direction {
                Direction::Outgoing => Node::ProperAccount(Account {
                    iban: ing_transaction.account,
                    name: String::from("My Account"),
                }),
                Direction::Incoming => determine_node_type(ing_transaction),
            }
        }

        fn determine_node_type(ing_transaction: &IngCurrentAccount) -> Node {
            if ing_transaction.code == Code::BA || ing_transaction.code == Code::GM {
                let termid = Regex::new(r"Term: (?<terminalID>\w+)").unwrap();
                let mut term_id_matcher = termid.captures_iter(&ing_transaction.description);
                return Node::Terminal(match term_id_matcher.next() {
                    Some(mtch) => mtch["terminalID"].into(),
                    None => "UNKNOWN_TERM_ID".into(),
                });
            }

            if ing_transaction.code == Code::DV {
                return Node::Other("ING".into());
            }

            if ing_transaction.code == Code::ST {
                return Node::Other("Deposit".into());
            }

            let my_account = Account {
                // TODO: Unhardcode
                iban: ing_transaction.account,
                name: String::from("My Account"),
            };
            if let Some(identifier) = &ing_transaction.counter_party {
                let brokerage = Regex::new(r"\d+").unwrap();

                if let Ok(iban) = Iban::parse(identifier) {
                    return Node::ProperAccount(Account {
                        iban,
                        name: String::from(&ing_transaction.name),
                    });
                } else if brokerage.is_match(identifier) {
                    return Node::SubAccount(SubAccount {
                        bsan: String::from(identifier),
                        name: String::from(&ing_transaction.name),
                        account_type: Some(AccountType::Brokerage),
                        parent_account: my_account,
                    });
                }
            }

            let o_spaarrekeningid =
                Regex::new(r"Oranje spaarrekening.*(?<sprekeningnr>[A-Z]\d+)").unwrap();
            let mut sprknr_id_matcher =
                o_spaarrekeningid.captures_iter(&ing_transaction.description);
            if let Some(sprknr) = sprknr_id_matcher.next() {
                return Node::SubAccount(SubAccount {
                    bsan: String::from(&sprknr["sprekeningnr"]),
                    name: String::from(&ing_transaction.name),
                    parent_account: my_account,
                    account_type: Some(AccountType::Saving),
                });
            }

            panic!(
                "Cannot determine sink for transaction {:?}",
                ing_transaction
            );
        }
    }
}

pub mod import {
    use super::serializers::ing::IngCurrentAccount;
    use crate::canonical::{state::Owner, transaction::*};
    use core::panic;
    use std::{
        collections::HashSet,
        error::Error,
        fs::{self, File},
        path,
    };

    /// Deserialize .csv files into a Vector of Transactions.
    ///
    /// `file_path` can point to:
    /// - a directory that contains at least 1 .csv file. In this case,
    /// all .csv's in that directory will be deserialized.
    /// - a single .csv file. In this case, just this .csv file will be
    /// deserialized.
    ///
    /// The resulting vector satisfies the following properties:
    /// - The transactions are sorted by increasing date, at the granularity
    /// of days. Order of transactions occuring on the same day cannot be guaranteed.
    /// - The transactions are unique. This is based on the hash of the transaction.
    /// Note: this isn't guaranteed to be the same hash ID you get from `Transaction::id()`.
    pub fn transactions_from_path(
        file_path: &path::PathBuf,
    ) -> Result<Transactions, Box<dyn Error>> {
        let files = match file_path {
            dirname if file_path.is_dir() => {
                println!("Looking for .csv files in directory: {:?}", dirname);
                let mut files: Vec<path::PathBuf> = vec![];
                for path in fs::read_dir(dirname)? {
                    let path = path.unwrap().path();
                    if path.is_file() && path.extension().unwrap() == "csv" {
                        files.push(path);
                    }
                }
                if files.is_empty() {
                    panic!("The directory: {:?} contains no .csv files.", dirname);
                }
                files
            }
            csv_file if file_path.is_file() && file_path.extension().unwrap() == "csv" => {
                vec![path::PathBuf::clone(csv_file)]
            }
            _ => panic!("Expecting a path to a directory or a .csv file"),
        };

        let mut transactions: Vec<Transaction> = Vec::new();
        println!("Reading:");
        for file in files {
            if let Ok(file) = File::open(file) {
                println!("> {:?}", file);
                transactions.append(&mut read_transactions_from(file));
            }
        }

        println!("Deduplicating transactions");
        let before = transactions.len();
        deduplicate(&mut transactions);
        println!(
            "> Removed {:?} duplicate transaction(s)",
            before - transactions.len()
        );

        println!("Sorting transactions on date");
        transactions.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(Transactions::new(transactions))
    }

    /// Deserialize the transactions in a single .csv file. At this point, there
    /// are no guarantees about uniqueness or order.
    ///
    /// Currently only supports hardcoded deserialization from `IngCurrentAccount`.
    fn read_transactions_from(file: File) -> Vec<Transaction> {
        let mut transactions: Vec<Transaction> = Vec::new();
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';') // Perhaps csv file specific.
            .flexible(true)
            .from_reader(file);
        for row in reader.deserialize::<IngCurrentAccount>().flatten() {
            transactions.push(Transaction::from(row));
        }
        transactions
    }

    /// Remove duplicate transactions from the vector. This leaves the transactions
    /// in random order.
    fn deduplicate(transactions: &mut Vec<Transaction>) -> &mut Vec<Transaction> {
        // itertools dedup.
        let set: HashSet<_> = transactions.drain(..).collect(); // dedup
        transactions.extend(set);
        transactions
    }

    pub fn profile_from_path(file_path: &path::PathBuf) -> Owner {
        use serde_json::from_reader;
        let profile_json_file = if file_path.is_file() && file_path.extension().unwrap() == "json" {
            file_path
        } else {
            panic!("Expecting a path to a .json file")
        };
        from_reader(File::open(profile_json_file).expect("File error")).unwrap()
    }
}
