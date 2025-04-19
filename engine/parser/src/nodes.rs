use asttree::*;

pub struct Nodes {
    pub nodes: Vec<Node>,
}

impl Nodes {
    pub fn empty() -> Nodes {
        Self { nodes: Vec::new() }
    }
    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
