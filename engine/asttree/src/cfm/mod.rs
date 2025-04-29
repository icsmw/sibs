mod gatekeeper;
mod skip;

pub use gatekeeper::*;
pub use skip::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum ControlFlowModifier {
    /// #[skip(param_a = "12", param_b = 12, func())]
    Gatekeeper(Gatekeeper),
    /// skip(func())
    /// skip(param_b = 12, func())
    /// skip(param_a = "12", param_b = 12, func())
    Skip(Skip),
}

impl Diagnostic for ControlFlowModifier {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            Self::Gatekeeper(n) => n.located(src, pos),
            Self::Skip(n) => n.located(src, pos),
        }
    }
    fn get_position(&self) -> Position {
        match self {
            Self::Gatekeeper(n) => n.get_position(),
            Self::Skip(n) => n.get_position(),
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            Self::Gatekeeper(n) => n.childs(),
            Self::Skip(n) => n.childs(),
        }
    }
}

impl ControlFlowModifier {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Gatekeeper(n) => &n.uuid,
            Self::Skip(n) => &n.uuid,
        }
    }
}

impl From<ControlFlowModifier> for Node {
    fn from(val: ControlFlowModifier) -> Self {
        Node::ControlFlowModifier(val)
    }
}

impl<'a> Lookup<'a> for ControlFlowModifier {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            Self::Gatekeeper(n) => n.lookup(trgs),
            Self::Skip(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for ControlFlowModifier {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::Gatekeeper(n) => n.find_mut_by_uuid(uuid),
            Self::Skip(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl SrcLinking for ControlFlowModifier {
    fn link(&self) -> SrcLink {
        match self {
            Self::Gatekeeper(n) => n.link(),
            Self::Skip(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        match self {
            Self::Gatekeeper(n) => n.slink(),
            Self::Skip(n) => n.slink(),
        }
    }
}
