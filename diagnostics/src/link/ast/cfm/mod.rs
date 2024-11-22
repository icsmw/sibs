mod gatekeeper;
mod skip;

use crate::*;
use asttree::*;

impl From<&ControlFlowModifier> for SrcLink {
    fn from(node: &ControlFlowModifier) -> Self {
        match node {
            ControlFlowModifier::Gatekeeper(n) => n.into(),
            ControlFlowModifier::Skip(n) => n.into(),
        }
    }
}
