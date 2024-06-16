use serde::{Deserialize, Serialize};

use crate::processing::types::Node;

// Example case: Tag my account as my account
#[derive(Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
    pub owns: Vec<Node>,
}
