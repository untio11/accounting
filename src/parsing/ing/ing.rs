use crate::{parsing::types::*, processing::types::*};
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
/// Transation.
#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct IngCurrentAccount {
    /// YYYYMMDD -> YYYY-MM-DD
    #[serde(rename = "Date", with = "local_date_deserializer")]
    pub date: NaiveDate,

    /// Name, often not very descriptive.
    #[serde(rename = "Name / Description")]
    pub name: String,

    /// XX00XXXX0000000000 -> IBAN, make a type for that. Always account of the owner?
    #[serde(rename = "Account", with = "serde_iban")]
    pub account: Iban,

    // IBAN, or [A-Z]?[0-9]{8} for ING sub-accounts (sparen, beleggen). Sparen actually empty, gotta extract from Notifications column...
    // Basically needs a post-processing step still. Do this when moving to universal transaction type.
    #[serde(rename = "Counterparty")]
    pub counter_party: Option<String>,

    /// Constant type from see https://nl.wikipedia.org/wiki/Rekeningafschrift
    #[serde(rename = "Code")]
    pub code: Code,

    /// Constant "Debit" | "Credit" - debit = incoming, credit = outgoing.
    #[serde(rename = "Debit/credit")]
    pub direction: Direction,

    /// 0000,00 - Always positive: sign depends on `self.direction`.
    #[serde(rename = "Amount (EUR)", with = "serde_amount")]
    pub amount: Decimal,

    /// Full name of `self.code`.
    #[serde(rename = "Transaction type")]
    pub transaction_type: String,

    /// Extra description as filled in by the initiator of the transaction.
    #[serde(rename = "Notifications")]
    pub description: String,

    /// 0000,00 - Balance of the account after this transaction
    #[serde(rename = "Resulting balance", with = "serde_amount")]
    pub balance: Decimal,

    /// Extra custom tags and/or text added by the account
    /// owner. -> Split to a set of #tags and a rest String.
    #[serde(rename = "Tag")]
    pub tags: String,
}

impl Transaction for IngCurrentAccount {
    fn amount(&self) -> Decimal {
        self.amount
    }

    fn date(&self) -> NaiveDate {
        self.date
    }

    fn description(&self) -> String {
        String::from(&self.description)
    }

    fn inherent_tags(&self) -> std::collections::HashSet<String> {
        // TODO: Figure out why we don't match #books at the end of the tags string.
        let reg = Regex::new(r"(#.+?)\b").unwrap();
        std::collections::HashSet::from_iter(
            reg.find_iter(&self.tags)
                .map(|m| String::from(m.as_str().trim())),
        )
    }

    fn sink(&self) -> Node {
        match self.direction {
            Direction::Incoming => Node::ProperAccount(Account {
                iban: self.account,
                name: String::from("My Account"),
                account_type: Some(AccountType::Checking), // TODO: Un-hardcode this -> from config
            }),
            Direction::Outgoing => determine_node_type(&self),
        }
    }

    fn source(&self) -> Node {
        match self.direction {
            Direction::Outgoing => Node::ProperAccount(Account {
                iban: self.account,
                name: String::from("My Account"),
                account_type: Some(AccountType::Checking), // TODO: Un-hardcode this.
            }),
            Direction::Incoming => determine_node_type(self),
        }
    }
}

fn determine_node_type(ing_transaction: &IngCurrentAccount) -> Node {
    let termid = Regex::new(r"Term: (?<terminalID>\w+)").unwrap();
    let mut term_id_matcher = termid.captures_iter(&ing_transaction.description);

    if ing_transaction.code == Code::BA {
        let mtch = term_id_matcher.next().unwrap();
        return Node::Terminal(String::from(&mtch["terminalID"]));
    } else if ing_transaction.code == Code::GM {
        let mtch = term_id_matcher.next().unwrap();
        return Node::ATM(String::from(&mtch["terminalID"]));
    }

    if let Some(identifier) = &ing_transaction.counter_party {
        let brokerage = Regex::new(r"\d+").unwrap();

        if let Ok(iban) = Iban::parse(identifier) {
            return Node::ProperAccount(Account {
                iban,
                name: String::from(&ing_transaction.name),
                account_type: None,
            });
        } else if brokerage.is_match(&identifier) {
            return Node::SubAccount(SubAccount {
                bsan: String::from(identifier),
                name: String::from(&ing_transaction.name),
                account_type: Some(AccountType::Brokerage),
                parent_account: Account {
                    // TODO: Unhardcode
                    iban: ing_transaction.account,
                    name: String::from("My Account"),
                    account_type: Some(AccountType::Checking),
                },
            });
        }
    }

    let o_spaarrekeningid = Regex::new(r"Oranje spaarrekening.*(?<sprekeningnr>[A-Z]\d+)").unwrap();
    let mut sprknr_id_matcher = o_spaarrekeningid.captures_iter(&ing_transaction.description);
    if let Some(sprknr) = sprknr_id_matcher.next() {
        return Node::SubAccount(SubAccount {
            bsan: String::from(&sprknr["sprekeningnr"]),
            name: String::from(&ing_transaction.name),
            parent_account: Account {
                iban: ing_transaction.account,
                name: String::from("My Account"),
                account_type: Some(AccountType::Checking),
            },
            account_type: Some(AccountType::Saving),
        });
    }

    panic!(
        "Cannot determine sink for transaction {:?}",
        ing_transaction
    );
}
