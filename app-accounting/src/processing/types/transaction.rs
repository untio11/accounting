use super::account::{Account, SubAccount};
use crate::{
    parsing::Direction,
    processing::{Identify, ID},
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// Represents a point between which money flows during transactions.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    Atm(String),
    /// Other, hard to identify nodes. For example: bank charges for services, deposits.
    Other(String),
}
impl Node {
    #[allow(dead_code)]
    fn name(&self) -> String {
        match self {
            Node::Atm(_) => String::from("ATM"),
            Node::ProperAccount(acc) => format!("(PA) {}", acc.name),
            Node::SubAccount(acc) => format!("(SA) {}", acc.name),
            Node::Terminal(_) => String::from("Payment Terminal"),
            Node::Other(id) => String::from(id),
        }
    }
    fn display_details(self) -> String {
        match &self {
            Node::Atm(id) => format!("^{}^ {} (ATM)", self.id(), id),
            Node::ProperAccount(acc) => format!("{}", acc),
            Node::SubAccount(acc) => format!("{}", acc),
            Node::Terminal(id) => format!("*{}* {} (Payment Terminal)", self.id(), id),
            Node::Other(id) => id.to_string(),
        }
    }
}
impl Identify for Node {
    type IdType = Node;
    fn id(&self) -> ID<Node> {
        let mut hasher = DefaultHasher::new();
        match self {
            Node::ProperAccount(acc) => Self::transfer_from(acc.id()),
            Node::SubAccount(acc) => Self::transfer_from(acc.id()),
            Node::Terminal(id) | Node::Atm(id) | Node::Other(id) => {
                id.hash(&mut hasher);
                ID(hasher.finish(), PhantomData)
            }
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().display_details())
    }
}

/// A uniform representation of monetary transactions, decoupled from the format provided
/// by the bank transaction exports.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Transaction {
    /// The date on which the transaction is registered.
    pub date: NaiveDate,
    /// The source of the money that is transferred in this transaction.
    pub source: Node,
    /// The destination of the money that is transferred in this transaction.
    pub sink: Node,
    /// The amount of money that is transferred in this transaction.
    pub amount: Decimal,
    /// A set of tags that can be derived directly from the data of the raw csv transaction.
    pub inherent_tags: Vec<String>,
    /// An inconsistantly formatted string describing some properties of the transaction.
    pub description: String,
}
impl Identify for Transaction {
    type IdType = Transaction;
}
impl Transaction {
    pub fn direction(&self, perspective: &Node) -> Direction {
        if self.sink.id() == perspective.id() {
            Direction::Incoming
        } else {
            Direction::Outgoing
        }
    }
}
