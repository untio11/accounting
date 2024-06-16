use crate::parsing::deserializers::serde_iban;
use crate::processing::{Identify, ID};
use core::fmt;
use iban::Iban;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
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
    // Deposit,
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
        ID(hasher.finish(), PhantomData)
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
        ID(hasher.finish(), PhantomData)
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
