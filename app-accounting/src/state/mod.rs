use crate::processing::{types::Node, Identify, ID};
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
