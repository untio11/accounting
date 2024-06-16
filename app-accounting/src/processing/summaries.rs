use super::types::{Node, Transactions};
use std::collections::HashMap;

pub fn node_frequencies(transactions: &Transactions) -> HashMap<&Node, u64> {
    let mut result = HashMap::new();
    for transaction in transactions {
        result
            .entry(&transaction.source)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
        result
            .entry(&transaction.sink)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }
    result
}
