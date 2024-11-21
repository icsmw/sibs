use crate::*;
use asttree::*;

impl ConflictResolver<ControlFlowModifierId> for ControlFlowModifierId {
    fn resolve_conflict(&self, _id: &ControlFlowModifierId) -> ControlFlowModifierId {
        match self {
            Self::Gatekeeper | Self::Skip => self.clone(),
        }
    }
}
