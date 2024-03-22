use crate::processing::Node;

pub struct Owner {
    pub name: String,
    pub owned: Vec<Node>,
}
