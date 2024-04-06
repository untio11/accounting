mod account;
mod transaction;

pub use account::*;
pub use transaction::{Node, Transaction};

/// A list of unique Transactions sorted by increasing date to the level
/// of days (i.e.: order of transactions on the same day cannot be guaranteed.)
pub type Transactions = Vec<Transaction>;
