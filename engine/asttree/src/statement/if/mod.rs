#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum IfCase {
    /// (LinkedNode::Expression::ComparisonSeq, LinkedNode::Statement::Block, Token)
    If(LinkedNode, LinkedNode, Token),
    /// (LinkedNode::Statement::Block, Token)
    Else(LinkedNode, Token),
}

impl IfCase {
    fn block(&self) -> &LinkedNode {
        match self {
            Self::If(n, ..) => n,
            Self::Else(n, ..) => n,
        }
    }
}

impl<'a> LookupInner<'a> for &'a IfCase {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            IfCase::If(con, blk, ..) => con
                .lookup_inner(owner, trgs)
                .into_iter()
                .chain(blk.lookup_inner(owner, trgs))
                .collect(),
            IfCase::Else(n, ..) => n.lookup_inner(owner, trgs),
        }
    }
}

impl FindMutByUuid for IfCase {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            IfCase::If(con, blk, ..) => con
                .find_mut_by_uuid(uuid)
                .or_else(|| blk.find_mut_by_uuid(uuid)),
            IfCase::Else(n, ..) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl fmt::Display for IfCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If(condition, block, _) => {
                    format!("{} {condition} {block} ", Keyword::If,)
                }
                Self::Else(block, _) => {
                    format!("{} {block} ", Keyword::Else,)
                }
            }
        )
    }
}

impl FindMutByUuid for Vec<IfCase> {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.iter_mut().find_map(|n| n.find_mut_by_uuid(uuid))
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub cases: Vec<IfCase>,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for If {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.cases
            .iter()
            .flat_map(|case| case.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for If {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.cases.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for If {
    fn link(&self) -> SrcLink {
        if let (Some(open), Some(close)) = (self.cases.first(), self.cases.last()) {
            src_from::nodes(open.block(), close.block())
        } else {
            SrcLink::new(&Uuid::default())
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.cases
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl From<If> for Node {
    fn from(val: If) -> Self {
        Node::Statement(Statement::If(val))
    }
}
