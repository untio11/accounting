mod account;
mod transaction;

pub use account::*;
pub use transaction::{Node, Transaction};

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
