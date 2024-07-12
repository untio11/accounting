pub mod transaction {
    use crate::canonical::{account::*, identify::*};
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::hash_map::DefaultHasher,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
    };

    /// A list of unique Transactions sorted by increasing date to the level
    /// of days (i.e.: order of transactions on the same day cannot be guaranteed.)
    pub struct Transactions(Vec<Transaction>);
    impl Transactions {
        pub fn iter(&self) -> std::slice::Iter<'_, Transaction> {
            self.0.iter()
        }
        pub fn new(transactions: Vec<Transaction>) -> Self {
            // TODO: Add sorting and validation here.
            Self(transactions)
        }
        /// Return a slice to view the full underlying vector.
        pub fn data(&self) -> &[Transaction] {
            &self.0
        }
    }

    /// Denotes the direction of the transaction.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Direction {
        /// AKA: "Credit", "Bij"
        Incoming,
        /// AKA: "Debit", "Af"
        Outgoing,
    }

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
                    ID::new(hasher.finish())
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
}

pub mod account {
    use super::identify::*;
    use crate::from_files::serializers::serde_iban;
    use core::fmt;
    use iban::Iban;
    use serde::{Deserialize, Serialize};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[derive(Hash, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
        Unknown,
    }
    /// A proper bank account that is guaranteed to have an Iban.
    #[derive(Hash, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct Account {
        #[serde(with = "serde_iban")]
        pub iban: Iban,
        pub name: String,
        // pub account_type: Option<AccountType>, // TODO: Can we actually derive this information from the raw data though?
    }
    impl Identify for Account {
        type IdType = Self;
        /// Just hash the iban for uniformity.
        fn id(&self) -> ID<Self> {
            let mut hasher = DefaultHasher::new();
            self.iban.hash(&mut hasher);
            ID::new(hasher.finish())
        }
    }
    impl fmt::Display for Account {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}] {}", self.id(), self.name,)
        }
    }

    /// Almost a bank account, except it's tied to a real account and as such doesn't have
    /// an official iban.
    #[derive(Hash, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct SubAccount {
        /// Bank Sub Account Number. This is not a "real" thing (I think), but it serves its purpose.
        pub bsan: String,
        pub name: String,
        pub parent_account: Account, // Might be nice to have? -> Also not generally known at read-time
        pub account_type: Option<AccountType>,
    }
    impl Identify for SubAccount {
        type IdType = Self;
        /// Just hash the bsan for uniformity.
        fn id(&self) -> ID<Self> {
            let mut hasher = DefaultHasher::new();
            self.bsan.hash(&mut hasher);
            ID::new(hasher.finish())
        }
    }
    impl fmt::Display for SubAccount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[{:?}/ {}] {}",
                self.parent_account.id(),
                self.id(),
                self.name
            )
        }
    }
}

pub mod state {
    use super::{identify::*, transaction::*};
    use serde::{Deserialize, Serialize};

    // Example case: Tag my account as my account
    #[derive(Serialize, Deserialize)]
    pub struct Owner {
        pub name: String,
        pub owns: Vec<Node>,
    }
    impl Owner {
        pub fn view(&self, id: &ID<Node>) -> Option<&Node> {
            self.owns.iter().find(|node| &node.id() == id)
        }
    }
}

pub mod identify {
    use std::{
        collections::hash_map::DefaultHasher,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        marker::PhantomData,
    };

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub struct ID<Of: Identify>(u64, PhantomData<Of>);
    impl<Of: Identify> ID<Of> {
        pub fn new(id: u64) -> Self {
            Self(id, PhantomData)
        }
    }
    impl<Of: Identify> From<ID<Of>> for String {
        fn from(val: ID<Of>) -> Self {
            format!("{:X}", val.0)
        }
    }
    /// Display the `u64` ID value as a hexadecimal string.
    ///
    /// E.g.: `"5E8C0A84534B0F04"`
    impl<Of: Identify> Debug for ID<Of> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:X}", self.0)
        }
    }
    impl<Of: Identify> Display for ID<Of> {
        /// Display the `u64` ID value as a hexadecimal string.
        ///
        /// E.g.: `"5E8C0A84534B0F04"`
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self) // Transfer Debug
        }
    }

    pub trait Identify
    where
        Self: Hash + Debug,
        Self::IdType: Identify,
    {
        type IdType;
        /// Generates a (somewhat?) consistent hash ID of self.
        /// Consistency is achieved by utilizing a fresh DefaultHasher
        /// for generating the hash.
        ///
        /// This claimed consistency has not been tested extensively
        /// at all lol.
        ///
        /// To implement id for your own type:
        /// ```
        /// fn id(&self) -> ID<Phantom> {
        ///     let mut hasher = DefaultHasher::new();
        ///     // Hash the relevant properties of self:
        ///     self.identifying_prop.hash(&mut hasher);
        ///     // ...
        ///     return ID(hasher.finish(), PhantomData);
        /// }
        /// ```
        fn id(&self) -> ID<Self::IdType> {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            ID(hasher.finish(), PhantomData)
        }

        /// Discard `other`'s ID Type and rewrap it in `Self::IdType`.
        ///
        /// Nice to have transitive ID's when subtypes define their own
        /// implementation of `.id()`.
        ///
        /// # Example
        /// `Account` is a value in `Node::ProperAccount(Account)` that
        /// defines its own implementation of `Identify.id()``, though
        /// their ID types differ, so `Node` can easily pass through
        /// the id value without recomputing it.
        /// ```
        /// let account_id: ID<Account> = Account::default().id();
        /// let node_id: ID<Node> = Node::transfer_from(account_id);
        /// ```
        fn transfer_from<OtherIdType: Identify>(other: ID<OtherIdType>) -> ID<Self::IdType> {
            match other {
                ID(other_id_value, _) => ID(other_id_value, PhantomData),
            }
        }
    }
}
