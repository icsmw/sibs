#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ComparisonGroup {
    pub open: Token,
    pub close: Token,
    pub node: Box<LinkedNode>,
    pub negation: Option<Token>,
    pub uuid: Uuid,
}

impl Diagnostic for ComparisonGroup {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.open.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::tokens(self.negation.as_ref().unwrap_or(&self.open), &self.close)
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&*self.node]
    }
}

impl<'a> Lookup<'a> for ComparisonGroup {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.node.lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for ComparisonGroup {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.node.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for ComparisonGroup {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.open, &self.close)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for ComparisonGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {} {}",
            self.negation
                .as_ref()
                .map(|tk| format!("{tk} "))
                .unwrap_or_default(),
            self.open,
            self.node,
            self.close
        )
    }
}

impl From<ComparisonGroup> for Node {
    fn from(val: ComparisonGroup) -> Self {
        Node::Expression(Expression::ComparisonGroup(val))
    }
}
