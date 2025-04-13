#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum CompoundAssignmentsOperator {
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
}

impl fmt::Display for CompoundAssignmentsOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompoundAssignmentsOperator::PlusEqual => Kind::PlusEqual,
                CompoundAssignmentsOperator::MinusEqual => Kind::MinusEqual,
                CompoundAssignmentsOperator::StarEqual => Kind::StarEqual,
                CompoundAssignmentsOperator::SlashEqual => Kind::SlashEqual,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct CompoundAssignmentsOp {
    pub token: Token,
    pub operator: CompoundAssignmentsOperator,
    pub uuid: Uuid,
}

impl CompoundAssignmentsOp {
    pub fn is_str_compatible(&self) -> bool {
        matches!(self.operator, CompoundAssignmentsOperator::PlusEqual)
    }
}

impl<'a> Lookup<'a> for CompoundAssignmentsOp {
    fn lookup(&'a self, _trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        vec![]
    }
}

impl FindMutByUuid for CompoundAssignmentsOp {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        None
    }
}

impl SrcLinking for CompoundAssignmentsOp {
    fn link(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for CompoundAssignmentsOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operator)
    }
}

impl From<CompoundAssignmentsOp> for Node {
    fn from(val: CompoundAssignmentsOp) -> Self {
        Node::Expression(Expression::CompoundAssignmentsOp(val))
    }
}
