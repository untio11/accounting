use std::collections::HashMap;

use super::{
    types::{Node, Transactions},
    Identify, ID,
};

pub fn node_frequencies(transactions: Transactions) -> HashMap<ID<Node>, u64> {
    let mut result = HashMap::new();
    for transaction in transactions {
        result
            .entry(transaction.id())
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }
    return result;
}
