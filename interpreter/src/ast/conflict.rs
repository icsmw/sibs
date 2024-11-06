use crate::*;

impl ConflictResolver<NodeId> for NodeId {
    fn resolve_conflict(&self, _id: &NodeId) -> NodeId {
        match self {
            Self::Statement
            | Self::Expression
            | Self::Declaration
            | Self::Value
            | Self::ControlFlowModifier
            | Self::Root
            | Self::Miscellaneous => self.clone(),
        }
    }
}
