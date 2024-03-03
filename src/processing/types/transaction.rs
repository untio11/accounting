use super::account::{Account, SubAccount};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::collections::HashSet;

/// Represents a the points between which money flows during transactions.
#[derive(Hash, Debug)]
pub enum Node {
    /// A fully qualified bank account with an IBAN.
    ProperAccount(Account),
    /// A sub-account to a `ProperAccount`. Doesn't have its own IBAN.
    /// Most likely a savings or brokerage account.
    SubAccount(SubAccount),
    /// A payment terminal in a shop or restaurant or something. You used
    /// your card or phone to pay something.
    Terminal(String),
    /// Make those numbers real and turn them into cold, hard cash.
    ATM(String),
}

/// A uniform representation of monetary transactions, decoupled from the format provided
/// by the bank transaction exports.
pub trait Transaction {
    /// The date on which the transaction is registered.
    fn date(&self) -> NaiveDate;
    /// The source of the money that is transferred in this transaction.
    fn source(&self) -> Node;
    /// The destination of the money that is transferred in this transaction.
    fn sink(&self) -> Node;
    /// The amount of money that is transferred in this transaction.
    fn amount(&self) -> Decimal;
    /// A set of tags that can be derived directly from the data of the raw csv transaction.
    fn inherent_tags(&self) -> HashSet<String>;
    /// An inconsistantly formatted string describing some properties of the transaction.
    fn description(&self) -> String;
}
