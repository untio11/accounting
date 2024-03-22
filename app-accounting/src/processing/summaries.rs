use super::{
    types::{Node, Transactions},
    Identify, ID,
};
use std::collections::HashMap;

pub fn node_frequencies(transactions: Transactions) -> HashMap<ID<Node>, u64> {
    let mut result = HashMap::new();
    for transaction in transactions {
        result
            .entry(transaction.source.id())
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
        result
            .entry(transaction.sink.id())
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    }
    return result;
}
