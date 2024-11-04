use crate::*;

impl ConflictResolver<ExpressionId> for ExpressionId {
    fn resolve_conflict(&self, _id: &ExpressionId) -> ExpressionId {
        // Variable and Comparing are in conflict
        match self {
            Self::Variable
            | Self::Comparing
            | Self::ComparingSeq
            | Self::Condition
            | Self::LogicalOp
            | Self::ComparisonOp
            | Self::Range
            | Self::BinaryExp
            | Self::Accessor
            | Self::Call
            | Self::FunctionCall
            | Self::Incrementer
            | Self::Command
            | Self::TaskCall => self.clone(),
        }
    }
}
