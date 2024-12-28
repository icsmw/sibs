use crate::*;

impl ConflictResolver<ExpressionId> for ExpressionId {
    fn resolve_conflict(&self, id: &ExpressionId) -> ExpressionId {
        match self {
            Self::ComparisonSeq
            | Self::ComparisonGroup
            | Self::BinaryExpSeq
            | Self::BinaryExpGroup => {
                if matches!(id, Self::Variable | Self::FunctionCall) {
                    id.to_owned()
                } else {
                    self.to_owned()
                }
            }
            Self::Variable
            | Self::Comparison
            | Self::LogicalOp
            | Self::ComparisonOp
            | Self::Range
            | Self::Accessor
            | Self::Call
            | Self::FunctionCall
            | Self::CompoundAssignments
            | Self::CompoundAssignmentsOp
            | Self::Command
            | Self::TaskCall
            | Self::BinaryOp
            | Self::BinaryExp => self.to_owned(),
        }
    }
}
