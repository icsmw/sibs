use lexer::SrcLink;

use crate::*;

impl From<&Expression> for SrcLink {
    fn from(node: &Expression) -> Self {
        match node {
            Expression::Accessor(n) => n.into(),
            Expression::BinaryExp(n) => n.into(),
            Expression::BinaryExpGroup(n) => n.into(),
            Expression::BinaryExpSeq(n) => n.into(),
            Expression::BinaryOp(n) => n.into(),
            Expression::Call(n) => n.into(),
            Expression::Command(n) => n.into(),
            Expression::Comparison(n) => n.into(),
            Expression::ComparisonGroup(n) => n.into(),
            Expression::ComparisonOp(n) => n.into(),
            Expression::ComparisonSeq(n) => n.into(),
            Expression::CompoundAssignments(n) => n.into(),
            Expression::CompoundAssignmentsOp(n) => n.into(),
            Expression::FunctionCall(n) => n.into(),
            Expression::LogicalOp(n) => n.into(),
            Expression::Range(n) => n.into(),
            Expression::TaskCall(n) => n.into(),
            Expression::Variable(n) => n.into(),
        }
    }
}
