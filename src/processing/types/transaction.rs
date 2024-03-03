use chrono::NaiveDate;
use iban::Iban;
use rust_decimal::Decimal;
use std::collections::HashSet;

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

// TODO: Move to own module for modeling account datatypes
#[derive(Hash, Debug)]
pub enum AccountType {
    /// Your everyday account: pay bills, buy stuff, receive salary. Cash is flowing here.
    /// No interest though most of the time.
    Checking,
    /// A place to save money. Less accessible, usually provides some interest.
    Saving,
    /// Also for saving money. Generally higher interest than savings account, but you
    /// sign a contract with the bank to leave the money there for a fixed amount of time.
    Deposit,
    /// This is where you put your money if you want to participate in the stock market.
    Brokerage,
}

/// A proper bank account that is guaranteed to have an Iban.
#[derive(Hash, Debug)]
pub struct Account {
    pub iban: Iban,
    pub name: String,
    pub account_type: Option<AccountType>, // TODO: Can we actually derive this information from the raw data though?
}

/// Almost a bank account, except it's tied to a real account and as such doesn't have
/// an official iban.
#[derive(Hash, Debug)]
pub struct SubAccount {
    /// Bank Sub Account Number. This is not a "real" thing (I think), but it serves
    // its purpose.
    pub bsan: String,
    pub name: String,
    pub parent_account: Account, // Might be nice to have?
    pub account_type: Option<AccountType>,
}
