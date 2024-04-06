use crate::processing::types::Node;

// Example case: Tag my account as my account
pub struct Owner<'a> {
    pub name: &'a str,
    pub owned: Vec<Node>,
}
