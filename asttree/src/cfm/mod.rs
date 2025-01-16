mod gatekeeper;
mod skip;

pub use gatekeeper::*;
pub use skip::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum ControlFlowModifier {
    /// #[skip([task_args], func())]
    /// #[skip([1, 2], func())]
    /// #[skip(["test", *], func())]
    /// #[skip([*,*], func())]
    Gatekeeper(Gatekeeper),
    /// skip([task_args], func())
    Skip(Skip),
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
