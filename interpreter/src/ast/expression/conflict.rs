use crate::*;

impl ConflictResolver<ExpressionId> for ExpressionId {
    fn resolve_conflict(&self, _id: &ExpressionId) -> ExpressionId {
        // Variable and Comparison are in conflict
        match self {
            Self::Variable
            | Self::Comparison
            | Self::ComparisonSeq
            | Self::ComparisonGroup
            | Self::Condition
            | Self::LogicalOp
            | Self::ComparisonOp
            | Self::Range
            | Self::BinaryExp
            | Self::Accessor
            | Self::Call
            | Self::FunctionCall
            | Self::CompoundAssignments
            | Self::CompoundAssignmentsOp
            | Self::Command
            | Self::TaskCall
            | Self::BinaryOp
            | Self::BinaryExpGroup
            | Self::BinaryExp
            | Self::BinaryExpSeq => self.clone(),
        }
    }
}
