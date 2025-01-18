#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Component {
    pub sig: Token,
    pub name: Token,
    pub path: String,
    pub nodes: Vec<LinkedNode>,
    pub open_bl: Token,
    pub close_bl: Token,
    pub uuid: Uuid,
}

impl Component {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl<'a> Lookup<'a> for Component {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl SrcLinking for Component {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.sig, &self.close_bl)
    }
    fn slink(&self) -> SrcLink {
        src_from::tks(&self.sig, &self.name)
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.sig,
            self.name,
            Kind::LeftParen,
            self.path,
            Kind::RightParen,
            self.open_bl,
            self.nodes
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.close_bl
        )
    }
}

impl From<Component> for Node {
    fn from(val: Component) -> Self {
        Node::Root(Root::Component(val))
    }
}
