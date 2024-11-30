use crate::*;
use asttree::*;

impl ConflictResolver<ExpressionId> for ExpressionId {
    fn resolve_conflict(&self, id: &ExpressionId) -> ExpressionId {
        // Variable and Comparison are in conflict
        match self {
            Self::Variable => {
                if matches!(id, ExpressionId::ComparisonSeq | Self::ComparisonGroup) {
                    self.to_owned()
                } else {
                    id.to_owned()
                }
            }
            Self::ComparisonSeq | Self::ComparisonGroup => {
                if matches!(id, ExpressionId::Variable) {
                    id.to_owned()
                } else {
                    self.to_owned()
                }
            }
            Self::Comparison
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
            | Self::BinaryExpGroup
            | Self::BinaryExp
            | Self::BinaryExpSeq => self.to_owned(),
        }
    }
}
