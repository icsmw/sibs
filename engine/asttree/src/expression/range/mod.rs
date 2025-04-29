#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Range {
    pub left: Box<LinkedNode>,
    pub right: Box<LinkedNode>,
    pub uuid: Uuid,
}

impl Diagnostic for Range {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.left.md.link.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.left.md.link.from(), self.right.md.link.to())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        vec![&*self.left, &*self.right]
    }
}

impl<'a> Lookup<'a> for Range {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.left
            .lookup_inner(self.uuid, trgs)
            .into_iter()
            .chain(self.right.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Range {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.left
            .find_mut_by_uuid(uuid)
            .or_else(|| self.right.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for Range {
    fn link(&self) -> SrcLink {
        src_from::nodes(&self.left, &self.right)
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, Kind::DotDot, self.right)
    }
}

impl From<Range> for Node {
    fn from(val: Range) -> Self {
        Node::Expression(Expression::Range(val))
    }
}
