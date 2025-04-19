#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Gatekeeper {
    pub token: Token,
    pub nodes: Vec<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for Gatekeeper {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.nodes
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for Gatekeeper {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.nodes.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for Gatekeeper {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.token, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.token,
            self.open,
            self.nodes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            self.close
        )
    }
}

impl From<Gatekeeper> for Node {
    fn from(val: Gatekeeper) -> Self {
        Node::ControlFlowModifier(ControlFlowModifier::Gatekeeper(val))
    }
}
