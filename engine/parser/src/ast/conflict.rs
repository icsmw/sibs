use crate::*;
use asttree::*;

impl ConflictResolver<NodeId> for NodeId {
    fn resolve_conflict(&self, id: &NodeId) -> NodeId {
        match self {
            Self::Statement | Self::Expression => {
                if matches!(id, Self::Value) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
            Self::Value
            | Self::Declaration
            | Self::ControlFlowModifier
            | Self::Root
            | Self::Miscellaneous => self.clone(),
        }
    }
}
