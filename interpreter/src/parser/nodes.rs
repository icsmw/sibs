use crate::*;

pub struct Nodes {
    pub nodes: Vec<Node>,
}

impl Nodes {
    pub fn empty() -> Nodes {
        Self { nodes: Vec::new() }
    }
}
